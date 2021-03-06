#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate clap;
extern crate fatcat;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;

use clap::{App, Arg};
#[allow(unused_imports)]
use fatcat::{
    AcceptEditgroupResponse, ApiError, ApiNoContext, ContextWrapperExt, CreateContainerBatchResponse, CreateContainerResponse, CreateCreatorBatchResponse, CreateCreatorResponse,
    CreateEditgroupResponse, CreateFileBatchResponse, CreateFileResponse, CreateReleaseBatchResponse, CreateReleaseResponse, CreateWorkBatchResponse, CreateWorkResponse, DeleteContainerResponse,
    DeleteCreatorResponse, DeleteFileResponse, DeleteReleaseResponse, DeleteWorkResponse, GetChangelogEntryResponse, GetChangelogResponse, GetContainerHistoryResponse, GetContainerResponse,
    GetCreatorHistoryResponse, GetCreatorReleasesResponse, GetCreatorResponse, GetEditgroupResponse, GetEditorChangelogResponse, GetEditorResponse, GetFileHistoryResponse, GetFileResponse,
    GetReleaseFilesResponse, GetReleaseHistoryResponse, GetReleaseResponse, GetStatsResponse, GetWorkHistoryResponse, GetWorkReleasesResponse, GetWorkResponse, LookupContainerResponse,
    LookupCreatorResponse, LookupFileResponse, LookupReleaseResponse, UpdateContainerResponse, UpdateCreatorResponse, UpdateFileResponse, UpdateReleaseResponse, UpdateWorkResponse,
};
#[allow(unused_imports)]
use futures::{future, stream, Future, Stream};

fn main() {
    let matches = App::new("client")
        .arg(
            Arg::with_name("operation")
                .help("Sets the operation to run")
                .possible_values(&[
                    "CreateContainerBatch",
                    "DeleteContainer",
                    "GetContainer",
                    "GetContainerHistory",
                    "LookupContainer",
                    "CreateCreatorBatch",
                    "DeleteCreator",
                    "GetCreator",
                    "GetCreatorHistory",
                    "GetCreatorReleases",
                    "LookupCreator",
                    "GetEditor",
                    "GetEditorChangelog",
                    "GetStats",
                    "AcceptEditgroup",
                    "GetChangelog",
                    "GetChangelogEntry",
                    "GetEditgroup",
                    "CreateFileBatch",
                    "DeleteFile",
                    "GetFile",
                    "GetFileHistory",
                    "LookupFile",
                    "CreateReleaseBatch",
                    "DeleteRelease",
                    "GetRelease",
                    "GetReleaseFiles",
                    "GetReleaseHistory",
                    "LookupRelease",
                    "CreateWorkBatch",
                    "DeleteWork",
                    "GetWork",
                    "GetWorkHistory",
                    "GetWorkReleases",
                ]).required(true)
                .index(1),
        ).arg(Arg::with_name("https").long("https").help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host").long("host").takes_value(true).default_value("localhost").help("Hostname to contact"))
        .arg(Arg::with_name("port").long("port").takes_value(true).default_value("8080").help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!(
        "{}://{}:{}",
        if is_https { "https" } else { "http" },
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    );
    let client = if is_https {
        // Using Simple HTTPS
        fatcat::Client::try_new_https(&base_url, "examples/ca.pem").expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        fatcat::Client::try_new_http(&base_url).expect("Failed to create HTTP client")
    };

    // Using a non-default `Context` is not required; this is just an example!
    let client = client.with_context(fatcat::Context::new_with_span_id(self::uuid::Uuid::new_v4().to_string()));

    match matches.value_of("operation") {
        // Disabled because there's no example.
        // Some("CreateContainer") => {
        //     let result = client.create_container(???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("CreateContainerBatch") => {
            let result = client.create_container_batch(&Vec::new(), Some(true), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("DeleteContainer") => {
            let result = client.delete_container("id_example".to_string(), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetContainer") => {
            let result = client.get_container("id_example".to_string(), Some("expand_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetContainerHistory") => {
            let result = client.get_container_history("id_example".to_string(), Some(789)).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("LookupContainer") => {
            let result = client.lookup_container("issnl_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("UpdateContainer") => {
        //     let result = client.update_container("id_example".to_string(), ???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        // Disabled because there's no example.
        // Some("CreateCreator") => {
        //     let result = client.create_creator(???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("CreateCreatorBatch") => {
            let result = client.create_creator_batch(&Vec::new(), Some(true), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("DeleteCreator") => {
            let result = client.delete_creator("id_example".to_string(), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetCreator") => {
            let result = client.get_creator("id_example".to_string(), Some("expand_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetCreatorHistory") => {
            let result = client.get_creator_history("id_example".to_string(), Some(789)).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetCreatorReleases") => {
            let result = client.get_creator_releases("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("LookupCreator") => {
            let result = client.lookup_creator("orcid_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("UpdateCreator") => {
        //     let result = client.update_creator("id_example".to_string(), ???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("GetEditor") => {
            let result = client.get_editor("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetEditorChangelog") => {
            let result = client.get_editor_changelog("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetStats") => {
            let result = client.get_stats(Some("more_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("AcceptEditgroup") => {
            let result = client.accept_editgroup("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("CreateEditgroup") => {
        //     let result = client.create_editgroup(???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("GetChangelog") => {
            let result = client.get_changelog(Some(789)).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetChangelogEntry") => {
            let result = client.get_changelog_entry(789).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetEditgroup") => {
            let result = client.get_editgroup("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("CreateFile") => {
        //     let result = client.create_file(???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("CreateFileBatch") => {
            let result = client.create_file_batch(&Vec::new(), Some(true), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("DeleteFile") => {
            let result = client.delete_file("id_example".to_string(), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetFile") => {
            let result = client.get_file("id_example".to_string(), Some("expand_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetFileHistory") => {
            let result = client.get_file_history("id_example".to_string(), Some(789)).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("LookupFile") => {
            let result = client.lookup_file("sha1_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("UpdateFile") => {
        //     let result = client.update_file("id_example".to_string(), ???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        // Disabled because there's no example.
        // Some("CreateRelease") => {
        //     let result = client.create_release(???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("CreateReleaseBatch") => {
            let result = client.create_release_batch(&Vec::new(), Some(true), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("CreateWork") => {
        //     let result = client.create_work(???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("DeleteRelease") => {
            let result = client.delete_release("id_example".to_string(), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetRelease") => {
            let result = client.get_release("id_example".to_string(), Some("expand_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetReleaseFiles") => {
            let result = client.get_release_files("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetReleaseHistory") => {
            let result = client.get_release_history("id_example".to_string(), Some(789)).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("LookupRelease") => {
            let result = client.lookup_release("doi_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("UpdateRelease") => {
        //     let result = client.update_release("id_example".to_string(), ???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        Some("CreateWorkBatch") => {
            let result = client.create_work_batch(&Vec::new(), Some(true), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("DeleteWork") => {
            let result = client.delete_work("id_example".to_string(), Some("editgroup_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetWork") => {
            let result = client.get_work("id_example".to_string(), Some("expand_example".to_string())).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetWorkHistory") => {
            let result = client.get_work_history("id_example".to_string(), Some(789)).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        Some("GetWorkReleases") => {
            let result = client.get_work_releases("id_example".to_string()).wait();
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        }

        // Disabled because there's no example.
        // Some("UpdateWork") => {
        //     let result = client.update_work("id_example".to_string(), ???, Some("editgroup_example".to_string())).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },
        _ => panic!("Invalid operation provided"),
    }
}
