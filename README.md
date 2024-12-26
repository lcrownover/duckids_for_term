# Get a List of DuckIDs for a Given Class

## Install

```bash
wget -O /tmp/duckids-for-term.tar.gz 'https://github.com/lcrownover/duckids_for_term/releases/download/v1.0.0/duckids_for_term.tar.gz'
tar xvzf /tmp/duckids_for_term.tar.gz -C /usr/local/bin
```

## Usage

```bash
export BANNER_API_KEY='your key'
duckids-for-term --term-code TERMCODE --crn CRN
```
