download tool in linux debian


apt install rustc

apt install git

git clone https://github.com/satlux666/lux-web.git

cd lux-web

cargo install --path .

cp /root/.cargo/bin/luxweb /usr/local/bin

chmod +x /usr/local/bin/luxweb
-----------------------------
luxweb

full scan 
luxweb full -t example.com


portscan
luxweb port -t example.com
domens
luxweb subs -t example.com
