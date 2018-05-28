#!/usr/bin/env python3

import argparse
from fatcat.api_client import FatCatApiClient

def import_crossref(args):
    fcc = FatCatApiClient(args.host_url)
    fcc.import_crossref_file(args.json_file,
        create_containers=args.create_containers)

def health(args):
    fcc = FatCatApiClient(args.host_url)
    print(fcc.health())

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--debug',
        action='store_true',
        help="enable debugging interface")
    parser.add_argument('--host-url',
        default="http://localhost:9411",
        help="connect to this host/port")
    subparsers = parser.add_subparsers()

    sub_import_crossref = subparsers.add_parser('import-crossref',
        aliases=['ic'])
    sub_import_crossref.set_defaults(func=import_crossref)
    sub_import_crossref.add_argument('json_file',
        help="crossref JSON file to import from")
    sub_import_crossref.add_argument('--create-containers',
        action='store_true',
        help="if true, create containers based on ISSN")

    sub_health = subparsers.add_parser('health')
    sub_health.set_defaults(func=health)

    args = parser.parse_args()
    args.func(args)

if __name__ == '__main__':
    main()
