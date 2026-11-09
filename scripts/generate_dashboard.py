import os
import re
import json
import html
import urllib.request
from collections import defaultdict
from datetime import datetime

REPO = os.environ["REPO"]
TOKEN = os.environ["GH_TOKEN"]

API = f"https://api.github.com/repos/{REPO}"

HEADERS = {
    "Authorization": f"Bearer {TOKEN}",
    "Accept": "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
}

BRANCH_PATTERN = re.compile(
    r"^(.+?)-(feat|fix|[a-z]+)-(.+)$"
)


def github_get(url):
    request = urllib.request.Request(url, headers=HEADERS)

    with urllib.request.urlopen(request) as response:
        return json.loads(response.read())


def get_all_pull_requests():
    pull_requests = []
    page = 1

    while True:
        url = (
            f"{API}/pulls"
            f"?state=all&per_page=100&page={page}"
        )

        batch = github_get(url)

        if not batch:
            break

        pull_requests.extend(batch)

        if len(batch) < 100:
            break

        page += 1

    return pull_requests


def parse_branch(branch):
    match = BRANCH_PATTERN.match(branch)

    if not match:
        return None

    return {
        "module": match.group(1),
        "type": match.group(2),
        "feature": match.group(3),
    }


def format_date(date):
    if not date:
        return "Unknown"

    return datetime.fromisoformat(
        date.replace("Z", "+00:00")
    ).strftime("%Y-%m-%d")


def generate_dashboard(pull_requests):
    modules = defaultdict(
        lambda: defaultdict(list)
    )

    for pr in pull_requests:
        branch = pr["head"]["ref"]

        parsed = parse_branch(branch)

        if not parsed:
            continue

        module = parsed["module"]
        branch_type = parsed["type"]

        modules[module][branch_type].append({
            "branch": branch,
            "feature": parsed["feature"],
            "title": pr["title"],
            "number": pr["number"],
            "url": pr["html_url"],
            "state": (
                "merged" if pr["merged_at"]
                else pr["state"]
            ),
            "date": pr["created_at"],
            "merged_at": pr["merged_at"],
        })

    output = []

    for module in sorted(modules):
        branches = modules[module]

        output.append(f"""
        <section class="module">
            <h2>{html.escape(module)}</h2>

            <div class="latest">
                {render_latest(branches, "feat")}
                {render_latest(branches, "fix")}
            </div>

            <h3>Branch history</h3>

            {render_history(branches)}
        </section>
        """)

    return "\n".join(output)


def render_latest(branches, branch_type):
    entries = branches.get(branch_type, [])

    if not entries:
        return f"""
        <div class="latest-card">
            <small>LATEST {branch_type.upper()}</small>
            <p>No branches found</p>
        </div>
        """

    latest = max(
        entries,
        key=lambda item: item["date"]
    )

    return f"""
    <div class="latest-card">
        <small>LATEST {branch_type.upper()}</small>
        <h3>{html.escape(latest['feature'])}</h3>

        <a href="{html.escape(latest['url'], quote=True)}">
            PR #{latest['number']}
        </a>

        <p>{format_date(latest['date'])}</p>
    </div>
    """


def render_history(branches):
    all_entries = []

    for entries in branches.values():
        all_entries.extend(entries)

    all_entries.sort(
        key=lambda item: item["date"],
        reverse=True
    )

    rows = []

    for item in all_entries:
        rows.append(f"""
        <tr>
            <td>{html.escape(item['branch'])}</td>
            <td>
                <a href="{html.escape(item['url'], quote=True)}">
                    #{item['number']} - {html.escape(item['title'])}
                </a>
            </td>
            <td>{html.escape(item['state'])}</td>
            <td>{format_date(item['date'])}</td>
        </tr>
        """)

    return f"""
    <table>
        <thead>
            <tr>
                <th>Branch</th>
                <th>Pull Request</th>
                <th>Status</th>
                <th>Created</th>
            </tr>
        </thead>
        <tbody>
            {''.join(rows)}
        </tbody>
    </table>
    """


def main():
    pull_requests = get_all_pull_requests()

    dashboard = generate_dashboard(pull_requests)

    os.makedirs("dist", exist_ok=True)

    with open("dist/index.html", "w") as file:
        file.write(f"""
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport"
                  content="width=device-width, initial-scale=1">

            <title>Module Dashboard</title>

            <style>
                body {{
                    font-family: Arial, sans-serif;
                    background: #f5f6fa;
                    color: #222;
                    margin: 0;
                    padding: 30px;
                }}

                h1 {{
                    margin-bottom: 30px;
                }}

                .module {{
                    background: white;
                    padding: 24px;
                    margin-bottom: 25px;
                    border-radius: 12px;
                    box-shadow: 0 2px 8px #0001;
                }}

                .latest {{
                    display: grid;
                    grid-template-columns:
                        repeat(auto-fit, minmax(200px, 1fr));
                    gap: 15px;
                }}

                .latest-card {{
                    background: #f5f6fa;
                    padding: 15px;
                    border-radius: 8px;
                }}

                .latest-card small {{
                    color: #666;
                    font-weight: bold;
                }}

                table {{
                    width: 100%;
                    border-collapse: collapse;
                    margin-top: 15px;
                }}

                th, td {{
                    padding: 12px;
                    text-align: left;
                    border-bottom: 1px solid #eee;
                }}

                a {{
                    color: #2563eb;
                    text-decoration: none;
                }}

                @media (max-width: 600px) {{
                    body {{ padding: 12px; }}
                    table {{ font-size: 12px; }}
                }}
            </style>
        </head>

        <body>
            <h1>Module Development Dashboard</h1>

            {dashboard}

        </body>
        </html>
        """)

    print("Dashboard generated successfully!")


if __name__ == "__main__":
    main()