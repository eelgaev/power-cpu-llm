"""Independent GGUF reader and NumPy dequantizers (test tooling only).

Written from the GGUF v3 container layout and the ggml block definitions
(block_q8_0, block_q5_0, block_q4_K, block_q6_K). It shares no code with the
engine's Rust/C readers or decoders. Weights are dequantized with the same
float32 operation order as ggml's scalar `dequantize_row_*` (product, then
offset), so values are the GGUF's defined F32 weights up to possible FMA
contraction in other builds (<= 1 ulp).
"""
import struct

import numpy as np

GGUF_TYPES = {0: "F32", 1: "F16", 2: "Q4_0", 6: "Q5_0", 8: "Q8_0", 12: "Q4_K", 14: "Q6_K"}
# (values per block, bytes per block)
BLOCKS = {"F32": (1, 4), "F16": (1, 2), "Q4_0": (32, 18), "Q8_0": (32, 34), "Q5_0": (32, 22), "Q4_K": (256, 144), "Q6_K": (256, 210)}
_SCALAR = {0: "<B", 1: "<b", 2: "<H", 3: "<h", 4: "<I", 5: "<i", 6: "<f", 7: "<?", 10: "<Q", 11: "<q", 12: "<d"}


class Tensor:
    def __init__(self, name, dims, kind, offset):
        self.name, self.dims, self.kind, self.offset = name, dims, kind, offset
        self.row_width = dims[0]
        self.rows = int(np.prod(dims[1:], dtype=np.int64)) if len(dims) > 1 else 1
        per, size = BLOCKS[kind]
        assert self.row_width % per == 0, (name, dims, kind)
        self.row_bytes = self.row_width // per * size


class GGUF:
    """Metadata parsed with `struct`; tensor bytes read through a read-only memmap."""

    def __init__(self, path):
        self.path = path
        with open(path, "rb") as f:
            magic, version, n_tensors, n_kv = struct.unpack("<4sIQQ", f.read(24))
            assert magic == b"GGUF" and version == 3, (magic, version)
            self.metadata = {}
            for _ in range(n_kv):
                key = self._string(f)
                self.metadata[key] = self._value(f, struct.unpack("<I", f.read(4))[0])
            self.tensors = {}
            for _ in range(n_tensors):
                name = self._string(f)
                n_dims = struct.unpack("<I", f.read(4))[0]
                dims = list(struct.unpack(f"<{n_dims}Q", f.read(8 * n_dims)))
                kind, offset = struct.unpack("<IQ", f.read(12))
                self.tensors[name] = Tensor(name, dims, GGUF_TYPES[kind], offset)
            align = self.metadata.get("general.alignment", 32)
            self.data_start = (f.tell() + align - 1) // align * align
        self.map = np.memmap(path, dtype=np.uint8, mode="r")

    @staticmethod
    def _string(f):
        (n,) = struct.unpack("<Q", f.read(8))
        return f.read(n).decode("utf-8")

    def _value(self, f, kind):
        if kind in _SCALAR:
            fmt = _SCALAR[kind]
            return struct.unpack(fmt, f.read(struct.calcsize(fmt)))[0]
        if kind == 8:
            return self._string(f)
        if kind == 9:
            item_kind, n = struct.unpack("<IQ", f.read(12))
            if item_kind in _SCALAR and item_kind != 7:
                fmt = _SCALAR[item_kind]
                width = struct.calcsize(fmt)
                return np.frombuffer(f.read(width * n), dtype=np.dtype(fmt)).copy()
            return [self._value(f, item_kind) for _ in range(n)]
        raise ValueError(f"unknown GGUF value type {kind}")

    def raw_rows(self, name, start, count):
        t = self.tensors[name]
        assert 0 <= start and start + count <= t.rows, (name, start, count, t.rows)
        begin = self.data_start + t.offset + start * t.row_bytes
        end = begin + count * t.row_bytes
        assert end <= self.map.shape[0], "tensor range beyond end of file"
        return np.asarray(self.map[begin:end]).reshape(count, t.row_bytes)

    def rows(self, name, start=0, count=None):
        """Dequantized float32 rows [count, row_width]."""
        t = self.tensors[name]
        count = t.rows - start if count is None else count
        return dequantize(self.raw_rows(name, start, count), t.kind, t.row_width)

    def matrix(self, name, chunk=4096):
        t = self.tensors[name]
        out = np.empty((t.rows, t.row_width), dtype=np.float32)
        for s in range(0, t.rows, chunk):
            n = min(chunk, t.rows - s)
            out[s:s + n] = self.rows(name, s, n)
        return out


def _f16(raw):
    return raw.copy().view(np.float16).astype(np.float32)


def dequantize(raw, kind, width):
    """raw: uint8 [rows, row_bytes] -> float32 [rows, width]."""
    rows = raw.shape[0]
    per, size = BLOCKS[kind]
    b = raw.reshape(rows * (width // per), size)
    if kind == "F32":
        return raw.copy().view("<f4").reshape(rows, width)
    if kind == "F16":
        return raw.copy().view("<f2").astype(np.float32).reshape(rows, width)
    if kind == "Q8_0":
        d = _f16(b[:, 0:2])
        q = b[:, 2:34].view(np.int8).astype(np.float32)
        y = q * d
    elif kind == "Q4_0":  # gguf-py: d * (nibble - 8); low nibbles 0..15, high 16..31
        d = _f16(b[:, 0:2])
        qs = b[:, 2:18]
        x0 = (qs & 0x0F).astype(np.int32) - 8
        x1 = (qs >> 4).astype(np.int32) - 8
        y = np.concatenate([x0, x1], axis=1).astype(np.float32) * d
    elif kind == "Q5_0":
        d = _f16(b[:, 0:2])
        qh = b[:, 2:6].copy().view("<u4").astype(np.uint32)  # [n, 1]
        qs = b[:, 6:22].astype(np.uint32)
        j = np.arange(16, dtype=np.uint32)
        xh0 = ((qh >> j) << 4) & 0x10
        xh1 = (qh >> (j + 12)) & 0x10
        x0 = ((qs & 0x0F) | xh0).astype(np.int32) - 16
        x1 = ((qs >> 4) | xh1).astype(np.int32) - 16
        y = np.concatenate([x0, x1], axis=1).astype(np.float32) * d
    elif kind == "Q4_K":
        d = _f16(b[:, 0:2])
        dmin = _f16(b[:, 2:4])
        s = b[:, 4:16].astype(np.uint8)
        sc = np.empty((b.shape[0], 8), dtype=np.uint8)
        mn = np.empty((b.shape[0], 8), dtype=np.uint8)
        sc[:, 0:4] = s[:, 0:4] & 63
        mn[:, 0:4] = s[:, 4:8] & 63
        sc[:, 4:8] = (s[:, 8:12] & 0x0F) | ((s[:, 0:4] >> 6) << 4)
        mn[:, 4:8] = (s[:, 8:12] >> 4) | ((s[:, 4:8] >> 6) << 4)
        qs = b[:, 16:144].reshape(-1, 4, 32)  # four 32-byte groups, each -> 64 values
        dsc = d * sc.astype(np.float32)  # [n, 8]  (d * sc) in f32
        mmn = dmin * mn.astype(np.float32)
        lo = (qs & 0x0F).astype(np.float32)
        hi = (qs >> 4).astype(np.float32)
        y = np.empty((b.shape[0], 4, 64), dtype=np.float32)
        y[:, :, :32] = dsc[:, 0::2, None] * lo - mmn[:, 0::2, None]
        y[:, :, 32:] = dsc[:, 1::2, None] * hi - mmn[:, 1::2, None]
    elif kind == "Q6_K":
        ql = b[:, 0:128].reshape(-1, 2, 64)
        qh = b[:, 128:192].reshape(-1, 2, 32)
        sc = b[:, 192:208].view(np.int8).astype(np.float32).reshape(-1, 2, 8)
        d = _f16(b[:, 208:210])[:, :, None]  # [n, 1, 1]
        q = np.empty((b.shape[0], 2, 4, 32), dtype=np.int32)
        q[:, :, 0] = ((ql[:, :, 0:32] & 0xF) | (((qh >> 0) & 3) << 4)).astype(np.int32) - 32
        q[:, :, 1] = ((ql[:, :, 32:64] & 0xF) | (((qh >> 2) & 3) << 4)).astype(np.int32) - 32
        q[:, :, 2] = ((ql[:, :, 0:32] >> 4) | (((qh >> 4) & 3) << 4)).astype(np.int32) - 32
        q[:, :, 3] = ((ql[:, :, 32:64] >> 4) | (((qh >> 6) & 3) << 4)).astype(np.int32) - 32
        # scale index: is = l // 16 within each 32-group, then +0/+2/+4/+6 per quarter
        l16 = np.arange(32) // 16
        scale = np.stack([sc[:, :, k * 2 + l16] for k in range(4)], axis=2)  # [n, 2, 4, 32]
        y = (d[:, :, :, None] * scale) * q.astype(np.float32)
    else:
        raise ValueError(kind)
    return y.astype(np.float32, copy=False).reshape(rows, width)
