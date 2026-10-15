#!/usr/bin/env python3
"""Long-context retrieval checks against a running `power-cpu-llm serve`.

    python3 tools/long-context-needle.py http://127.0.0.1:18080 \\
        --tokens 8000 16000 --where start middle end --report out.json

A unique code is placed at the start, middle or end of a long filler document; the request asks for it.
Pass criterion (fixed in advance): HTTP 200 and the exact code appears in the answer. The prompt size is
approximate (about 30 tokens per filler sentence); the server's usage.prompt_tokens is reported.
Behavioural checks only: no quality, accuracy-rate or throughput claims.
"""
import argparse
import json
import sys
import time
import urllib.error
import urllib.request

SENTENCES = [
    "The archive room held shelves of maintenance logs describing routine inspections of pumps and valves.",
    "Each entry noted the inspector, the weather, and whether the pressure readings matched the previous week.",
    "Most pages were unremarkable, recording small leaks, replaced gaskets and the occasional faulty gauge.",
    "In winter the logs grew shorter, as crews spent their shifts clearing snow from the access roads.",
]


def document(tokens, where, code):
    count = max(4, tokens // 22)
    body = [SENTENCES[i % len(SENTENCES)] for i in range(count)]
    needle = f"Important: the vault access code is {code}."
    index = {"start": 0, "middle": count // 2, "end": count}[where]
    body.insert(index, needle)
    return " ".join(body)


def ask(url, text, max_tokens):
    body = {"messages": [{"role": "user", "content": text + "\n\nWhat is the vault access code mentioned in the "
                          "document above? Answer with the code only."}], "max_completion_tokens": max_tokens}
    request = urllib.request.Request(url + "/v1/chat/completions", data=json.dumps(body).encode(),
                                     headers={"content-type": "application/json"})
    start = time.time()
    try:
        with urllib.request.urlopen(request, timeout=24 * 3600) as response:
            return response.status, response.read().decode(), time.time() - start
    except urllib.error.HTTPError as error:
        return error.code, error.read().decode(), time.time() - start


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("url")
    ap.add_argument("--tokens", type=int, nargs="+", default=[8000])
    ap.add_argument("--where", nargs="+", default=["start", "middle", "end"])
    ap.add_argument("--report")
    args = ap.parse_args()
    results = []
    for n, tokens in enumerate(args.tokens):
        for m, where in enumerate(args.where):
            code = f"ORCA-{4000 + 37 * n + 11 * m}-{'XYZ'[m % 3]}"
            status, raw, seconds = ask(args.url, document(tokens, where, code), 64)
            answer, prompt_tokens = "", None
            try:
                value = json.loads(raw)
                answer = value["choices"][0]["message"]["content"]
                prompt_tokens = value["usage"]["prompt_tokens"]
            except (KeyError, ValueError, TypeError):
                pass
            ok = status == 200 and code in answer
            results.append({"target_tokens": tokens, "where": where, "code": code, "status": status,
                            "prompt_tokens": prompt_tokens, "seconds": round(seconds, 1), "answer": answer,
                            "pass": ok, "raw": raw if not ok else None})
            print(f"{'PASS' if ok else 'FAIL'} ~{tokens} tokens ({prompt_tokens}) needle at {where}: "
                  f"{answer!r} [{status}, {seconds:.0f}s]", flush=True)
            if args.report:
                json.dump(results, open(args.report, "w"), indent=1)
    failed = [r for r in results if not r["pass"]]
    print(json.dumps({"cases": len(results), "failed": len(failed)}))
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
