## How to reproduce issue

Run test-setup.sh

Output before using override: 

``` File "test_dir"
File "test_dir"
File "test_dir/other_nested_dir"
File "test_dir/other_nested_dir/file1.txt"
File "test_dir/nested_dir"
File "test_dir/nested_dir/file1.txt"
File "test_dir/nested_dir/file.txt"
File "test_dir/nested_dir/file2.txt"
File "test_dir/file.txt"
```

Expected output after using override:

```File "test_dir"
File "test_dir"
File "test_dir/other_nested_dir"
File "test_dir/other_nested_dir/file1.txt"
File "test_dir/nested_dir"
File "test_dir/nested_dir/file1.txt"
File "test_dir/nested_dir/file.txt"
File "test_dir/nested_dir/file2.txt"
File "test_dir/file.txt"
File "test_dir/.hidden_dir/file.txt"
File "test_dir/.hidden_dir/.hidden_file.txt"
```

Actual output after using override:

```
File "test_dir"
File "test_dir/other_nested_dir"
File "test_dir/nested_dir"
File "test_dir/.hidden_dir"
```