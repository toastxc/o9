mkdir ~/.o9/
mkdir ~/.o9/mi-create/
cd ~/.o9/ || exit
git clone https://github.com/ooflet/Mi-Create.git mi-create/
cd mi-create
python -m pip install -r requirements.txt
cd ..
printf "#!/bin/bash\npython3 ~/.o9/mi-create/src/main.py \$@" > ~/.o9/mi-create_bin
chmod a+x ~/.o9/mi-create_bin