complete -c machconvert -s p -d 'copy on new source with a file prefix' -r
complete -c machconvert -s c -d 'color (priority 1)' -r -f -a "{grayscale	''}"
complete -c machconvert -s f -d 'flip (priority 2)' -r -f -a "{horizontal	'',vertical	''}"
complete -c machconvert -s r -d 'rotate (priority 3) with degree. 90, 180 or 270.' -r -f -a "{90	'',180	'',270	''}"
complete -c machconvert -s h -l help -d 'Print help'
complete -c machconvert -s V -l version -d 'Print version'
