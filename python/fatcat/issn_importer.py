
import sys
import json
import itertools
import fatcat_client
from fatcat.importer_common import FatcatImporter

# CSV format (generated from git.archive.org/webgroup/oa-journal-analysis):
# ISSN-L,in_doaj,in_road,in_norwegian,in_crossref,title,publisher,url,lang,ISSN-print,ISSN-electronic,doi_count,has_doi,is_oa,is_kept,publisher_size,url_live,url_live_status,url_live_final_status,url_live_final_url,url_live_status_simple,url_live_final_status_simple,url_domain,gwb_pdf_count

def or_none(s):
    if s is None:
        return None
    if len(s) == 0:
        return None
    return s

def truthy(s):
    if s is None:
        return None
    s = s.lower()
    if s in ('true', 't', 'yes', 'y', '1'):
        return True
    elif s in ('false', 'f', 'no', 'n', '0'):
        return False
    else:
        return None

class FatcatIssnImporter(FatcatImporter):

    def parse_issn_row(self, row):
        """
        row is a python dict (parsed from CSV).
        returns a ContainerEntity
        """
        title = or_none(row['title'])
        issnl = or_none(row['ISSN-L'])
        if title is None or issnl is None:
            return
        extra = dict(
            in_doaj=truthy(row['in_doaj']),
            in_road=truthy(row['in_road']),
            in_norwegian=truthy(row['in_norwegian']),
            language=or_none(row['lang']),
            url=or_none(row['url']),
            ISSNp=or_none(row['ISSN-print']),
            ISSNe=or_none(row['ISSN-electronic']),
            is_oa=truthy(row['is_oa']),
            is_kept=truthy(row['is_kept']),
        )
        ce = fatcat_client.ContainerEntity(
            issnl=issnl,
            name=title,
            publisher=or_none(row['publisher']),
            abbrev=None,
            coden=None,
            extra=extra)
        return ce

    def create_row(self, row, editgroup=None):
        ce = self.parse_issn_row(row)
        if ce is not None:
            self.api.create_container(ce, editgroup=editgroup)
            self.insert_count = self.insert_count + 1

    def create_batch(self, batch, editgroup=None):
        """Reads and processes in batches (not API-call-per-line)"""
        objects = [self.parse_issn_row(l)
                   for l in batch if l != None]
        objects = [o for o in objects if o != None]
        self.api.create_container_batch(objects, autoaccept="true", editgroup=editgroup)
        self.insert_count = self.insert_count + len(objects)
