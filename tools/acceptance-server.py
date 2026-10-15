#!/usr/bin/env python3
"""Behavioural acceptance checks against a running `power-cpu-llm serve`.

    python3 tools/acceptance-server.py http://127.0.0.1:18080 --report out.json

Each case has a pass criterion fixed in advance (below). These are pass/fail behaviour checks of the
served model + engine, not quality or throughput claims. Exit status 1 if any case fails.
"""
import argparse
import json
import re
import sys
import time
import urllib.error
import urllib.request


def post(url, body, timeout=3600):
    request = urllib.request.Request(url + "/v1/chat/completions", data=json.dumps(body).encode(),
                                     headers={"content-type": "application/json"})
    start = time.time()
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            raw = response.read().decode()
            status = response.status
    except urllib.error.HTTPError as error:
        raw, status = error.read().decode(), error.code
    return status, raw, time.time() - start


def content(raw):
    return json.loads(raw)["choices"][0]["message"].get("content", "")


def user(text, **extra):
    return {"messages": [{"role": "user", "content": text}], **extra}


def counting_ok(text, n):
    return [int(x) for x in re.findall(r"\d+", text)] == list(range(1, n + 1))


def not_degenerate(text):
    words = text.split()
    grams = [" ".join(words[i:i + 5]) for i in range(len(words) - 4)]
    return len(words) >= 150 and all(grams.count(g) <= 3 for g in set(grams))


NEEDLE = "The secret launch code is PELICAN-7319."
FILLER = ("The archive room held shelves of old maintenance logs, each describing routine inspections of "
          "pumps, valves and wiring in the northern facility. ")


def cases():
    long_text = NEEDLE + " " + FILLER * 140 + "\nWhat is the secret launch code mentioned at the very start? Answer with the code only."
    return [
        ("capital", user("What is the capital of France? Answer with one word."),
         lambda r: r["status"] == 200 and content(r["raw"]).strip().strip(".") == "Paris"),
        ("arithmetic", user("What is 17 multiplied by 23? Answer with just the number."),
         lambda r: r["status"] == 200 and "391" in content(r["raw"])),
        ("system_prompt", {"messages": [
            {"role": "system", "content": "Always answer in French."},
            {"role": "user", "content": "What color is the sky on a clear day? Answer in one short sentence."}]},
         lambda r: r["status"] == 200 and re.search(r"\bbleu", content(r["raw"]).lower()) is not None),
        ("multi_turn", {"messages": [
            {"role": "user", "content": "My name is Alex and my favourite fruit is mango."},
            {"role": "assistant", "content": "Nice to meet you, Alex! Mango is a great choice."},
            {"role": "user", "content": "What is my favourite fruit? Answer with one word."}]},
         lambda r: r["status"] == 200 and "mango" in content(r["raw"]).lower()),
        ("translation", user("Translate 'good morning' into Spanish. Answer with the translation only."),
         lambda r: r["status"] == 200 and "buenos d" in content(r["raw"]).lower()),
        ("primes", user("List the first five prime numbers as a comma-separated list, nothing else."),
         lambda r: r["status"] == 200 and re.sub(r"\s", "", content(r["raw"])).strip(".") == "2,3,5,7,11"),
        ("counting_100", user("List every integer from 1 through 100 in increasing order, separated by single spaces.",
                              max_completion_tokens=512),
         lambda r: r["status"] == 200 and counting_ok(content(r["raw"]), 100)),
        ("code", user("Write a Python function named factorial that returns n! for a non-negative integer n."),
         lambda r: r["status"] == 200 and "def factorial" in content(r["raw"])),
        ("story_multicanvas", user("Write a story of about 400 words about a lighthouse keeper.",
                                   max_completion_tokens=768),
         lambda r: r["status"] == 200 and not_degenerate(content(r["raw"]))),
        ("reasoning_exposed", user("What is 2 + 2? Answer with just the number.", include_reasoning=True),
         lambda r: r["status"] == 200 and "4" in content(r["raw"])
         and "reasoning_content" in json.loads(r["raw"])["choices"][0]["message"]),
        ("long_prompt_needle", user(long_text, max_completion_tokens=64),
         lambda r: r["status"] == 200 and "PELICAN-7319" in content(r["raw"])),
        ("over_limit_rejected", user("word " * 5000),
         lambda r: r["status"] == 400 and json.loads(r["raw"])["error"]["code"] == "context_length_exceeded"),
    ]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("url")
    ap.add_argument("--report")
    ap.add_argument("--only", nargs="*")
    args = ap.parse_args()
    results = []
    for name, body, check in cases():
        if args.only and name not in args.only:
            continue
        status, raw, seconds = post(args.url, body)
        result = {"name": name, "status": status, "seconds": round(seconds, 2), "raw": raw}
        try:
            result["pass"] = bool(check(result))
        except Exception as error:  # malformed response counts as a failure
            result["pass"] = False
            result["check_error"] = repr(error)
        shown = raw if len(raw) < 400 else raw[:400] + "..."
        print(f"{'PASS' if result['pass'] else 'FAIL'} {name} [{status}, {seconds:.1f}s] {shown}", flush=True)
        results.append(result)
    if args.report:
        json.dump(results, open(args.report, "w"), indent=1)
    failed = [r["name"] for r in results if not r["pass"]]
    print(json.dumps({"cases": len(results), "failed": failed}))
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
