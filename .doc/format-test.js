const http = require('http');

const content = `cpu=[45.6]
mem=[78]
swap=[12.3]
io_read=[125.6MB/s]
io_write=[88MB/s]
net_rx=[1024.5KB/s]
net_tx=[768.2TB/s]`;

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain; charset=utf-8' });
    res.end(content);
});

server.listen(3000, () => {
    console.log('serving at http://localhost:3000');
});