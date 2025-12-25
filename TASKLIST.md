{
  "project": {
    "name": "UBL 2 ∞ — Universal Business Ledger on Container Universal + TDLN",
    "mission": "Registrar, em prova criptográfica, qualquer fato de negócio, garantindo que só artefatos autorizados cruzem fronteiras (ALLOW/DENY determinístico) e que toda decisão seja reproduzível fora do cluster.",
    "value_prop": [
      "✅ Veredito sub-milissegundo para qualquer objeto (ZIP, API, LLM)",
      "🛡️ Ledger imutável, Merkle-root ancorada externamente",
      "🔒 Passkey 2-eyes para toda concessão de poder (Permit)",
      "🪶 Escopo mínimo via Hop-Token (blast-radius ≪)",
      "📜 Auditoria completa sem acesso on-line (slice + root proof)"
    ]
  },

  "dependencies": {
    "languages": ["Rust 1.74", "TypeScript 5.x (Node 20)", "SQL (PostgreSQL 15 / SQLite 3.42)"],
    "crates_core": ["serde", "blake3", "ed25519-dalek", "axum", "tokio", "moka", "sqlx", "wasmtime"],
    "npm": ["simple-webauthn", "pest", "esbuild", "tsx"],
    "tooling": ["cargo-nextest", "cargo-tarpaulin", "trivy", "semgrep", "sigstore/cosign", "Grafana/Tempo"],
    "services": {
      "PostgreSQL": "ledger_events, ledger_roots",
      "Redis (opcional)": "revocation & replay-cache",
      "Git (bare repo)": "ancorar Merkle-root diário",
      "Grafana": "dashboards latência/denies",
      "CI": "GitHub Actions matrix x86_64/aarch64"
    }
  },

  "sprints": [
    {
      "id": 0,
      "name": "Bootstrap & Legacy Purge",
      "duration_days": 5,
      "stories": [
        {
          "title": "Inicializar repo + CI",
          "tasks": [
            "Criar monorepo e rust-toolchain.toml",
            "Workflow build.yml (cargo check)",
            "Badge \"build passing\" no README"
          ],
          "done_only_if": [
            "CI green em commit main",
            "Branch-protection ativo"
          ]
        },
        {
          "title": "Purgar DELETE e arquivar HIBERNATE",
          "tasks": [
            "Rodar tools/purge.js DELETE.txt",
            "Mover HIBERNATE → archived/ com feature `legacy`",
            "Executar npm test (541 verdes)"
          ],
          "done_only_if": [
            "Conteúdo KEEP intacto",
            "Nenhum teste legado falha"
          ]
        }
      ]
    },

    {
      "id": 1,
      "name": "Deterministic Kernel",
      "duration_days": 10,
      "stories": [
        {
          "title": "Crate ubl-kernel",
          "tasks": [
            "Implementar canonical_json (Json✯Atomic)",
            "Funções blake3_hash(domain, bytes)",
            "ed25519_sign / verify",
            "Enum EventDomain"
          ],
          "done_only_if": [
            "cargo clippy --deny warnings limpa",
            "Cobertura tarpaulin ≥ 90%",
            "Fuzz quickcheck 10k iterações sem pânico"
          ]
        }
      ]
    },

    {
      "id": 2,
      "name": "Ledger Engine",
      "duration_days": 10,
      "stories": [
        {
          "title": "Schema & invariantes",
          "tasks": [
            "Criar tabela ledger_events (aggregate, sequence, hashes, payload)",
            "Trigger SQL nega UPDATE/DELETE",
            "Função append() valida Genesis, Hash, Sequence, Domain"
          ],
          "done_only_if": [
            "append_genesis_ok test passa",
            "reject_sequence_gap falha como esperado",
            "hash mismatch rejeitado"
          ]
        },
        {
          "title": "Merkle-root diário",
          "tasks": [
            "Job tokio_cron midnight",
            "Calcular root, gravar ledger_roots",
            "git tag ledger-YYYY-MM-DD <root>"
          ],
          "done_only_if": [
            "Script verify_root reproduz hash",
            "Tag push em repositório bare"
          ]
        }
      ]
    },

    {
      "id": 3,
      "name": "Membrana Fast-Path",
      "duration_days": 10,
      "stories": [
        {
          "title": "Serviço /verify",
          "tasks": [
            "Rota Axum POST /verify -> Bytes",
            "Integrar ubl-kernel::verify",
            "Replay-cache LRU (moka)",
            "Emitir Decision para Ledger"
          ],
          "done_only_if": [
            "Benchmark wrk2 10k rps p95 ≤ 1 ms",
            "Decision aparece no slice ledger"
          ]
        }
      ]
    },

    {
      "id": 4,
      "name": "Wallet & Permit",
      "duration_days": 10,
      "stories": [
        {
          "title": "Passkey 2-eyes Vault",
          "tasks": [
            "Fluxo simple-webauthn",
            "Endpoint POST /permit",
            "Revogação /permit/:jti"
          ],
          "done_only_if": [
            "Permit TTL ≤ 900 s",
            "Revoked Permit => Membrana DENY",
            "CLI 'ubl permit approve' funciona"
          ]
        }
      ]
    },

    {
      "id": 5,
      "name": "Policy Engine (TDLN → WASM)",
      "duration_days": 10,
      "stories": [
        {
          "title": "DSL → Wasm determinístico",
          "tasks": [
            "Especificar gramática .tdln (pest)",
            "Compilar para Wasm via wasm-encoder",
            "Embed Wasmtime em Membrana"
          ],
          "done_only_if": [
            "Mesmo .tdln gera idêntico policy_hash",
            "Gas-meter aborta >100k fuel",
            "Fuzz 24 h sem crash"
          ]
        }
      ]
    },

    {
      "id": 6,
      "name": "Runner & Receipt",
      "duration_days": 10,
      "stories": [
        {
          "title": "Runner isolado",
          "tasks": [
            "Namespace network none + seccomp",
            "Pull /next com Permit",
            "Receipt (stdout/stderr hash + exit)"
          ],
          "done_only_if": [
            "exec.start → exec.finish cadeia íntegra",
            "Duplicate receipt ⇒ DENY_DUPLICATE"
          ]
        }
      ]
    },

    {
      "id": 7,
      "name": "Portal & Observability",
      "duration_days": 10,
      "stories": [
        {
          "title": "Portal premium",
          "tasks": [
            "SvelteKit init, Tailwind dark/light",
            "MDX docs de rustdoc",
            "Playground envelope",
            "Provision Grafana dashboards"
          ],
          "done_only_if": [
            "Lighthouse score 100/100",
            "Playground mostra ALLOW/DENY em <300 ms",
            "Dashboard latência e denies ativo"
          ]
        }
      ]
    }
  ],

  "global_principles": {
    "security": "trivy, cargo-audit, semgrep critical = 0",
    "signing": "cosign todos os binários + CycloneDX SBOM",
    "coverage_thresholds": { "kernel": 90, "services": 75 },
    "release_tags": ["v0.7.0-beta0", "v0.8.0-beta1", "v0.9.0-rc1", "v1.0.0-ga"],
    "done_only_if": [
      "CI verde em main",
      "Cobertura ≥ thresholds",
      "Benchmarks meta atendidas",
      "Audit Oracle valida Merkle-root diário"
    ]
  }
}
