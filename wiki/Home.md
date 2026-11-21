# SigmaOS-buck

SigmaOS integration component for **Buck2 build system**.

## Commands

```bash
sigmaos-buck install   # install package + deploy defaults
sigmaos-buck deploy    # copy config templates to $HOME
sigmaos-buck status    # binary/package/config report
sigmaos-buck reset     # remove deployed config
sigmaos-buck open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
