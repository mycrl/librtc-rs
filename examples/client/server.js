const { WebSocketServer } = require('ws');
const { createServer } = require('http');
const express = require('express');
const url = require('url');

const app = express();
const server = createServer(app);

// This is used here to serve a sample webpage via an http server.
app.get('/', (_, res) => {
    res.sendFile(__dirname + '/examples.html');
});

/**
 * The websocket connection pool stores all websocket sessions, 
 * the key is the remote port number, and the value is the 
 * `WebSocket` class.
 */
let sockets = {};

new WebSocketServer({ server })
    .on('connection', (socket, req) => {
        /**
         * Triggered when a new websocket is connected to the 
         * server, the remote port number of the session is 
         * obtained here and stored in the connection pool.
         */
        const id = String(req.socket.remotePort);
        console.log('connected for : ' + id);
        sockets[id] = socket;
        
        /**
         * Process all messages of the current connection, 
         * and broadcast the received messages to all 
         * connections except itself.
         */
        socket.on('message', payload => {
            Object.keys(sockets)
                .filter(key => key != id)
                .forEach(key => sockets[key].send(payload.toString()));
        });

        /**
         * Removes the current session from the connection 
         * pool when the current connection is disconnected.
         */
        socket.on('close', () => {
            delete sockets[id];
        });
    });

// Start the http server and listen on port 80.
server.listen(80, () => {
    console.log('\r\n\r\n');
    console.log('signaling server starting...');
    console.log('web page: http://localhost');
    console.log('signaling: ws://localhost');
    console.log('\r\n\r\n');
});