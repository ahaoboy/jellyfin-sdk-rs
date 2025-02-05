# ServerConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**log_file_retention_days** | Option<**i32**> | Gets or sets the number of days we should retain log files. | [optional]
**is_startup_wizard_completed** | Option<**bool**> | Gets or sets a value indicating whether this instance is first run. | [optional]
**cache_path** | Option<**String**> | Gets or sets the cache path. | [optional]
**previous_version** | Option<**String**> | Gets or sets the last known version that was ran using the configuration. | [optional]
**previous_version_str** | Option<**String**> | Gets or sets the stringified PreviousVersion to be stored/loaded,  because System.Version itself isn't xml-serializable. | [optional]
**enable_metrics** | Option<**bool**> | Gets or sets a value indicating whether to enable prometheus metrics exporting. | [optional]
**enable_normalized_item_by_name_ids** | Option<**bool**> |  | [optional]
**is_port_authorized** | Option<**bool**> | Gets or sets a value indicating whether this instance is port authorized. | [optional]
**quick_connect_available** | Option<**bool**> | Gets or sets a value indicating whether quick connect is available for use on this server. | [optional]
**enable_case_sensitive_item_ids** | Option<**bool**> | Gets or sets a value indicating whether [enable case sensitive item ids]. | [optional]
**disable_live_tv_channel_user_data_name** | Option<**bool**> |  | [optional]
**metadata_path** | Option<**String**> | Gets or sets the metadata path. | [optional]
**metadata_network_path** | Option<**String**> |  | [optional]
**preferred_metadata_language** | Option<**String**> | Gets or sets the preferred metadata language. | [optional]
**metadata_country_code** | Option<**String**> | Gets or sets the metadata country code. | [optional]
**sort_replace_characters** | Option<**Vec<String>**> | Gets or sets characters to be replaced with a ' ' in strings to create a sort name. | [optional]
**sort_remove_characters** | Option<**Vec<String>**> | Gets or sets characters to be removed from strings to create a sort name. | [optional]
**sort_remove_words** | Option<**Vec<String>**> | Gets or sets words to be removed from strings to create a sort name. | [optional]
**min_resume_pct** | Option<**i32**> | Gets or sets the minimum percentage of an item that must be played in order for playstate to be updated. | [optional]
**max_resume_pct** | Option<**i32**> | Gets or sets the maximum percentage of an item that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched. | [optional]
**min_resume_duration_seconds** | Option<**i32**> | Gets or sets the minimum duration that an item must have in order to be eligible for playstate updates.. | [optional]
**min_audiobook_resume** | Option<**i32**> | Gets or sets the minimum minutes of a book that must be played in order for playstate to be updated. | [optional]
**max_audiobook_resume** | Option<**i32**> | Gets or sets the remaining minutes of a book that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched. | [optional]
**library_monitor_delay** | Option<**i32**> | Gets or sets the delay in seconds that we will wait after a file system change to try and discover what has been added/removed  Some delay is necessary with some items because their creation is not atomic.  It involves the creation of several  different directories and files. | [optional]
**image_saving_convention** | Option<[**models::ImageSavingConvention**](ImageSavingConvention.md)> |  | [optional]
**metadata_options** | Option<[**Vec<models::MetadataOptions>**](MetadataOptions.md)> |  | [optional]
**skip_deserialization_for_basic_types** | Option<**bool**> |  | [optional]
**server_name** | Option<**String**> |  | [optional]
**ui_culture** | Option<**String**> |  | [optional]
**save_metadata_hidden** | Option<**bool**> |  | [optional]
**content_types** | Option<[**Vec<models::NameValuePair>**](NameValuePair.md)> |  | [optional]
**remote_client_bitrate_limit** | Option<**i32**> |  | [optional]
**enable_folder_view** | Option<**bool**> |  | [optional]
**enable_grouping_into_collections** | Option<**bool**> |  | [optional]
**display_specials_within_seasons** | Option<**bool**> |  | [optional]
**codecs_used** | Option<**Vec<String>**> |  | [optional]
**plugin_repositories** | Option<[**Vec<models::RepositoryInfo>**](RepositoryInfo.md)> |  | [optional]
**enable_external_content_in_suggestions** | Option<**bool**> |  | [optional]
**image_extraction_timeout_ms** | Option<**i32**> |  | [optional]
**path_substitutions** | Option<[**Vec<models::PathSubstitution>**](PathSubstitution.md)> |  | [optional]
**enable_slow_response_warning** | Option<**bool**> | Gets or sets a value indicating whether slow server responses should be logged as a warning. | [optional]
**slow_response_threshold_ms** | Option<**i64**> | Gets or sets the threshold for the slow response time warning in ms. | [optional]
**cors_hosts** | Option<**Vec<String>**> | Gets or sets the cors hosts. | [optional]
**activity_log_retention_days** | Option<**i32**> | Gets or sets the number of days we should retain activity logs. | [optional]
**library_scan_fanout_concurrency** | Option<**i32**> | Gets or sets the how the library scan fans out. | [optional]
**library_metadata_refresh_concurrency** | Option<**i32**> | Gets or sets the how many metadata refreshes can run concurrently. | [optional]
**remove_old_plugins** | Option<**bool**> | Gets or sets a value indicating whether older plugins should automatically be deleted from the plugin folder. | [optional]
**allow_client_log_upload** | Option<**bool**> | Gets or sets a value indicating whether clients should be allowed to upload logs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


