# Server Deployment

The game uses 2 services, both deployed to an EC2 instance:

- web server. This is used to serve the static web site consisting of the WebAssembly target, the JavaScript bindings, and the index HTML. Plus the assets folder.
- [matchbox_server](https://crates.io/crates/matchbox_server). This is the signaling server used to match 2 players and hand off a P2P session.

## EC2 Server

The EC2 server was created from an Amazon Linux 2 AMI (as recommended by Bing Copilot). (I tried also an Azure VM, but hit an obstacle when trying to install Matchbox Server so switched over to AWS.) I used the Lexia AWS Playground account.

- Instance name: [jorge-matchbox](https://387084687389-eywknzqv.us-east-2.console.aws.amazon.com/ec2/home?region=us-east-2#InstanceDetails:instanceId=i-0a7ed04e2730c43da)
- Instance ID: i-0a7ed04e2730c43da
- Public IP: 3.147.199.67
- Key pair: Jorge-matchbox.pem
- Security: AWS security group allows inbound traffic from anywhere on ports 80 (HTTP), 443 (HTTPS), 22 (SSH), 3536 (for matchbox server)
- Package manager: YUM

SSH:

```
ssh -i "jorge-matchbox.pem" ec2-user@ec2-3-147-199-67.us-east-2.compute.amazonaws.com
```

Note: can also connect via Session Manager (since I installed that agent and set up the role).

### Installation Notes

To install rust:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Web Server

[http://3.147.199.67/](http://3.147.199.67/)

To build:

```
cargo build --target wasm32-unknown-unknown --release
```

To generate bindings:

```
wasm-bindgen --out-dir static --target web target/wasm32-unknown-unknown/release/dungeon_crawl_p2p.wasm
```

To deploy:

```
scp -i ~/.ssh/jorge-matchbox.pem -r \
    static \
    assets \
    ec2-user@ec2-3-147-199-67.us-east-2.compute.amazonaws.com:/var/www/dungeon_crawler_p2p/
```

### Installation Notes

- `sudo chown ec2-user:ec2-user /var/www/dungeon_crawler_p2p`
- `sudo chown -R nginx:nginx /var/www/dungeon_crawler_p2p/`
- `sudo chmod -R 755 /var/www/dungeon_crawler_p2p/`
- `sudo nano /etc/nginx/nginx.conf`
- `sudo systemctl enable nginx`
- `sudo systemctl start nginx`
- `sudo yum install -y iptables iptables-services`
- `sudo systemctl enable iptables`
- `sudo systemctl start iptables`
- `sudo nano /etc/sysconfig/iptables`
- `sudo systemctl restart iptables`

- The `nginx.conf` is critical. The server block is the only change and it needs to specify both the static and asset dirs.
- The directories and all the files need to have the proper permissions.
- The IP tables are also critical, including the order of the rules.

Here are the IP tables:

`sudo iptables -L INPUT -n --line-numbers`

```
Chain INPUT (policy ACCEPT)
num  target     prot opt source               destination
1    ACCEPT     tcp  --  0.0.0.0/0            0.0.0.0/0            tcp dpt:3536
2    ACCEPT     tcp  --  0.0.0.0/0            0.0.0.0/0            tcp dpt:80
3    ACCEPT     all  --  0.0.0.0/0            0.0.0.0/0            state RELATED,ESTABLISHED
4    ACCEPT     icmp --  0.0.0.0/0            0.0.0.0/0
5    ACCEPT     all  --  0.0.0.0/0            0.0.0.0/0
6    ACCEPT     tcp  --  0.0.0.0/0            0.0.0.0/0            state NEW tcp dpt:22
7    REJECT     all  --  0.0.0.0/0            0.0.0.0/0            reject-with icmp-host-prohibited
8    ACCEPT     all  --  0.0.0.0/0            0.0.0.0/0            state RELATED,ESTABLISHED
9    ACCEPT     all  --  0.0.0.0/0            0.0.0.0/0
10   REJECT     all  --  0.0.0.0/0            127.0.0.0/8          reject-with icmp-port-unreachable
11   ACCEPT     tcp  --  0.0.0.0/0            0.0.0.0/0            tcp dpt:443
12   DROP       all  --  0.0.0.0/0            0.0.0.0/0
```

## Matchbox Server

Set up to run as a service on port 3536. The app is configured to connect to this server via `MATCHBOX_ROOM_URL` (which should be set to "ws://3.147.199.67:3536/dungeon_crawl?next=2").

This is only to create a 2-player pair that then talk to each other directly during game-play.

### Installation Notes

After installing Rust, use cargo install to install the server. To run as a service:

- `sudo nano /etc/systemd/system/matchbox_server.service`
- `sudo systemctl daemon-reload`
- `sudo systemctl enable matchbox_server`
- `sudo systemctl start matchbox_server`

(All above based on info from Bing Copilot)
