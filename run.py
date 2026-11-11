#!/usr/bin/env python3
"""
🌌 Divine Pseudoword WhatsApp Username Generator
Uses Gemini or Groq API — zero external dependencies (stdlib only)
"""

import os
import json
import urllib.request
import urllib.error
import sys

MASTER_PROMPT = """# 🌌 THE OMNISCIENT LINGUIST DIRECTIVE: THE GENESIS OF PSEUDOWORDS

## Role & Persona
You are a Divine Linguistic Architect, an omniscient master of letters, phonotactics,
and auditory aesthetics. You possess total knowledge of every language, dialect, and slang
ever spoken by humanity. Your purpose is not to use this knowledge to speak, but to AVOID
it entirely. You are the creator of "Absolute Pseudowords"—strings of characters that
perfectly mimic the aesthetic, rhythmic, and phonetic structure of real human words, yet
exist completely devoid of meaning, history, or earthly usage.

## Core Objective
Generate elegant, pronounceable, and natural-feeling character assemblies (pseudowords)
that have absolutely ZERO meaning, translation, or etymological root in English or any
other known human language.

## The Absolute Laws of Assembly (Must Follow Strictly):

### 1. The Law of Phonotactics (Natural Flow)
- NO RANDOM STRINGS: Do not generate keyboard smashes (e.g., xqrzt, asdfgh).
- PRONOUNCEABILITY: The assemblies must flow naturally off the human tongue. Use logical
  consonant-vowel structures (CVC, CCVC, CVCC, CVCV).
- SYLLABIC HARMONY: Blend vowels and consonants in a way that implies they could belong
  to a highly evolved, beautiful, but completely alien language.

### 2. The Law of the Semantic Void (Total Meaninglessness)
- ZERO MEANING: Must not translate to anything in any human language.
- NO ROOT WORDS: No morphemes, prefixes, or suffixes (-tion, -ism, -ing, pre-, anti-).
- NO NAMES OR SLANG: Not names of places, fictional characters, or modern slang.

### 3. The Law of Unfamiliarity (The Never Heard Principle)
- If it sounds like a real word or heavily implies one, discard it.
- Must feel like a phantom language—structurally sound, yet completely alien.

## Task:
Generate exactly 2 completely different pseudoword WhatsApp usernames.
Include a phonetic pronunciation guide for each.

## Output Format (follow exactly):
1. Username: <assembly> | Pronunciation: [<phonetic guide>]
2. Username: <assembly> | Pronunciation: [<phonetic guide>]"""


def call_gemini(api_key: str) -> str:
    url = (
        "https://generativelanguage.googleapis.com/v1beta/models/"
        f"gemini-3.8-flash:generateContent?key={api_key}"
    )
    payload = {
        "system_instruction": {"parts": [{"text": MASTER_PROMPT}]},
        "contents": [{
            "role": "user",
            "parts": [{"text": "Generate 2 pseudoword WhatsApp usernames now."}]
        }],
        "generationConfig": {"temperature": 1.1}
    }
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        url, data=data,
        headers={"Content-Type": "application/json"}
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        result = json.loads(resp.read().decode())
    return result["candidates"][0]["content"]["parts"][0]["text"].strip()


def call_groq(api_key: str) -> str:
    url = "https://api.groq.com/openai/v1/chat/completions"
    payload = {
        "model": "llama-3.3-70b-versatile",
        "messages": [
            {"role": "system", "content": MASTER_PROMPT},
            {"role": "user", "content": "Generate 2 pseudoword WhatsApp usernames now."}
        ],
        "temperature": 1.1
    }
    data = json.dumps(payload).encode()
    req = urllib.request.Request(
        url, data=data,
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {api_key}"
        }
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        result = json.loads(resp.read().decode())
    return result["choices"][0]["message"]["content"].strip()


def pick_provider():
    gemini_key = os.environ.get("GEMINI_API_KEY", "")
    groq_key = os.environ.get("GROQ_API_KEY", "")

    if gemini_key:
        return "gemini", gemini_key
    if groq_key:
        return "groq", groq_key

    print("\nNo API key found in environment. Choose provider:")
    print("  1) Google Gemini  (recommended)")
    print("  2) Groq")
    choice = input("Enter 1 or 2 [default: 1]: ").strip() or "1"
    provider = "groq" if choice == "2" else "gemini"
    key = input(f"Paste your {'Gemini' if provider == 'gemini' else 'Groq'} API key: ").strip()
    return provider, key


def generate(provider: str, key: str) -> str:
    if provider == "gemini":
        return call_gemini(key)
    return call_groq(key)


def main():
    print("=" * 52)
    print("  🌌 Divine Pseudoword WhatsApp Username Generator")
    print("=" * 52)

    provider, key = pick_provider()

    while True:
        print(f"\n⚡ Generating via {'Gemini 3.8 Flash' if provider == 'gemini' else 'Groq Llama 70B'}...")
        try:
            output = generate(provider, key)
            print("\n" + output)
        except urllib.error.HTTPError as e:
            body = e.read().decode()
            print(f"\n❌ HTTP {e.code}: {body}")
        except Exception as e:
            print(f"\n❌ Error: {e}")

        print("\n" + "-" * 52)
        cmd = input("Press Enter to retry | p = change key/provider | q = quit: ").strip().lower()
        if cmd in ("q", "quit"):
            print("Stay divine. Goodbye!")
            break
        elif cmd == "p":
            provider, key = pick_provider()


if __name__ == "__main__":
    main()
