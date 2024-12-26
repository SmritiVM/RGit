I want to model git commit as list
give the code that creates a Commit structure in Rust 
the Commit structure should be modelled as a list of CommitObjects
each commit object / node in the list should have the following - commit id, index hash, commit message

Write a function create commit that does the following
the commit id should be calculated by reading the .wegit/HEAD file. The HEAD file contains just a number, commit id is one plus the number in HEAD. HEAD gets updated to hold the new commit id.
the index hash is calculated by reading .wegit/index. Call the calculate_sha1 function from the hash_and_compress module to calculate hash_code
commit message is a string parameter that is passed
.wegit/commit should store all the commitobjects
After the commit object is created, it should be appended .wegit/commit and stored there.
