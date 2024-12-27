# Structures and Utils

Separate structures and utils have been created to modularize code and handle repeated calls.

## Structures

###  Index
An `Index` object is a HashMap of `IndexObjects`. The structure of an index object is as follows:
*`filepath`
*`hash_code`
The functions for the `IndexObject` include `add_index_object`, `read_index`, `write_index`

### Commit
A `Commit` object has the following fields:
*`index_hash`
*`commit_message`

The functions include `create_commit` and `read_commits`

### Paths
Just a list of constant paths that are often used in the code

## Utils

### Hash Compress
The public functions are as follows:
*`create_object`
*`retrieve_object`

These internally call the following:
*`calculate_sha1`
*`compress_object`
*`decompress_object`

### Message Handler
All the errors, logging messages are passed to this. The `handle_message` function prints it out to console (or can be changed based on the requirement)