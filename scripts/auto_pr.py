import os
import json
import sys
from pathlib import Path
from groq import Groq
import urllib.request
import urllib.error

MAX_DIFF_CHARS = 60000
MODEL = "llama-3.3-70b-versatile"


def create_pr(token, repo, head_branch, base_branch, title, body):
    url = f"https://api.github.com/repos/{repo}/pulls"
    data = json.dumps({
        "title": title,
        "body": body,
        "head": head_branch,
        "base": base_branch,
    }).encode()
    req = urllib.request.Request(url, data=data, method="POST")
    req.add_header("Authorization", f"Bearer {token}")
    req.add_header("Content-Type", "application/json")
    req.add_header("Accept", "application/vnd.github+json")
    try:
        with urllib.request.urlopen(req) as resp:
            result = json.loads(resp.read())
            print(f"PR created: {result['html_url']}")
    except urllib.error.HTTPError as e:
        error_body = e.read().decode()
        print(f"Error creating PR: {e.code} {error_body}", file=sys.stderr)
        sys.exit(1)


def main():
    groq_api_key = os.environ["GROQ_API_KEY"]
    gh_pat = os.environ["GH_PAT"]
    head_branch = os.environ["HEAD_BRANCH"]
    base_branch = os.environ.get("BASE_BRANCH", "main")
    repo = os.environ["REPO"]
    commit_log = os.environ.get("COMMIT_LOG", "")

    diff = Path("diff.txt").read_text(encoding="utf-8", errors="replace")
    if not diff.strip():
        print("No diff found")
        sys.exit(1)

    if len(diff) > MAX_DIFF_CHARS:
        diff = diff[:MAX_DIFF_CHARS] + "\n\n... (diff truncated)"

    client = Groq(api_key=groq_api_key)

    prompt = f"""あなたはPull Requestの説明文を書くアシスタントです。
以下のgit diffとコミットログをもとに、PRのタイトルと説明文を生成してください。

コミットログ:
{commit_log}

Diff:
```diff
{diff}
```

以下のJSON形式で回答してください（他のテキストは不要）:
{{"title": "PRタイトル（70文字以内）", "body": "## 概要\\n- ...\\n\\n## 変更内容\\n- ...\\n\\n## 確認事項\\n- [ ] ..."}}"""

    response = client.chat.completions.create(
        model=MODEL,
        messages=[{"role": "user", "content": prompt}],
        max_tokens=1500,
    )

    content = response.choices[0].message.content.strip()

    if "```json" in content:
        content = content.split("```json")[1].split("```")[0].strip()
    elif "```" in content:
        content = content.split("```")[1].split("```")[0].strip()

    result = json.loads(content)
    title = result["title"]
    body = result["body"] + "\n\n---\n🤖 Generated with Groq AI"

    create_pr(gh_pat, repo, head_branch, base_branch, title, body)


if __name__ == "__main__":
    main()
