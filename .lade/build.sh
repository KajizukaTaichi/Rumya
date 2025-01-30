interpreter_path=$(realpath "rumya.lm")
printf "#!/bin/bash\nlamuta $interpreter_path \$1" > rumya.sh
chmod +x rumya.sh
