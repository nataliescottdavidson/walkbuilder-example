#!/bin/bash

mkdir test_dir
mkdir test_dir/nested_dir
mkdir test_dir/.hidden_dir
mkdir test_dir/other_nested_dir
mkdir test_dir/.other_hidden_dir
touch test_dir/file.txt
touch test_dir/.hidden_file.txt
touch test_dir/nested_dir/file1.txt
touch test_dir/nested_dir/file2.txt
touch test_dir/nested_dir/.hidden_file.txt
touch test_dir/.hidden_dir/file.txt
touch test_dir/.hidden_dir/.hidden_file.txt
touch test_dir/.other_hidden_dir/file.txt
touch test_dir/.other_hidden_dir/.hidden_file.txt
touch test_dir/other_nested_dir/file1.txt
touch test_dir/nested_dir/file2.txt
touch test_dir/nested_dir/.hidden_file.txt