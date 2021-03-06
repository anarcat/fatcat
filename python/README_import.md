
Run in order:

- ISSN
- ORCID
- Crossref
- Manifest

Lots of trouble with encoding; always `export LC_ALL=C.UTF-8`

Start off with:

    sudo su webcrawl
    cd /srv/fatcat/src/python
    export LC_ALL=C.UTF-8
    pipenv shell
    export LC_ALL=C.UTF-8

## Data Sources

Download the following; uncompress the sqlite file, but **do not** uncompress
the others:

    https://archive.org/download/crossref_doi_dump_201801/crossref-works.2018-01-21.json.xz
    https://archive.org/download/ia_papers_manifest_2018-01-25/index/idents_files_urls.sqlite.gz
    https://archive.org/download/ia_journal_metadata_explore_2018-04-05/journal_extra_metadata.csv
    https://archive.org/download/issn_issnl_mappings/20180216.ISSN-to-ISSN-L.txt
    https://archive.org/download/orcid-dump-2017/public_profiles_API-2.0_2017_10_json.tar.gz
    https://archive.org/download/ia_journal_pid_map_munge_20180908/release_ids.ia_munge_20180908.sqlite3.gz
    https://archive.org/download/ia_test_paper_matches/2018-08-27-2352.17-matchcrossref.insertable.json.gz
    https://archive.org/download/ia_papers_manifest_2018-01-25_matched/ia_papers_manifest_2018-01-25.matched.json.gz

## ISSN

From CSV file:

    # See "start off with" command above
    time ./fatcat_import.py import-issn /srv/fatcat/datasets/journal_extra_metadata.csv

Usually a couple minutes at most on fast production machine.

## ORCID

Usually tens of minutes on fast production machine.

    time parallel --bar --pipepart -j8 -a /srv/fatcat/datasets/public_profiles_1_2_json.all.json ./fatcat_import.py import-orcid -

## Crossref

Usually 24 hours or so on fast production machine.

    time xzcat /srv/fatcat/datasets/crossref-works.2018-09-05.json.xz | time parallel -j20 --round-robin --pipe ./fatcat_import.py import-crossref - /srv/fatcat/datasets/20180216.ISSN-to-ISSN-L.txt /srv/fatcat/datasets/release_ids.ia_munge_20180908.sqlite3

## Matched

Unknown speed!

    # No file update for the first import...
    zcat /srv/fatcat/datasets/ia_papers_manifest_2018-01-25.matched.json.gz | pv -l | time parallel -j12 --round-robin --pipe ./fatcat_import.py import-matched --no-file-update -

    # ... but do on the second
    zcat /srv/fatcat/datasets/2018-08-27-2352.17-matchcrossref.insertable.json.gz | pv -l | time parallel -j12 --round-robin --pipe ./fatcat_import.py import-matched -

    # GROBID extracted (release+file)
    time zcat /srv/fatcat/datasets/2018-09-23-0405.30-dumpgrobidmetainsertable.longtail_join.filtered.tsv.gz | pv -l | time parallel -j12 --round-robin --pipe ./fatcat_import.py import-grobid-metadata -
