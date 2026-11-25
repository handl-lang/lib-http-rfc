# lib-http-rfc

`http_rfc` — HTTP/1.1 wire codec in pure handl: request parse +
response serialize. The wire layer under `serve`/`serve-epoll`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the codec — request record `{ method, target, path, query, version, headers, body, body_bytes }`, response record `{ status?, headers?, body? \| body_bytes? }` |
| `pkg.hnl` | manifest — `modules: ["http_rfc"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Pure `.hnl`, no Rust crate; the parsed-request/response record
  shapes are the contract every HTTP-layer lib (`http-router`,
  `http-middleware`, `http-static`) codes against.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
