complete -c machreduce -s o -d 'output to a specific file (like result.zip)' -r -F
complete -c machreduce -s d -d 'direction : horizontal or vertical (vertical by default)' -r -f -a "{horizontal\t'',vertical\t''}"
complete -c machreduce -s h -l help -d 'Print help'
complete -c machreduce -s V -l version -d 'Print version'
