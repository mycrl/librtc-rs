const { WebSocketServer } = require('ws')
const { createServer } = require('http')
const express = require('express')
const url = require('url')

const app = express()
const server = createServer(app)

app.get('/', (_, res) => {
    res.sendFile(__dirname + '/index.html')
})

let sockets = {}

new WebSocketServer({ server })
    .on('connection', (socket, req) => {
        const id = String(req.socket.remotePort)
        console.log('connected for : ' + id)
        sockets[id] = socket

        socket.on('message', payload => {
            Object.keys(sockets)
                .filter(key => key != id)
                .forEach(key => sockets[key].send(payload.toString()))
        })

        socket.on('close', () => {
            delete sockets[id]
        })
    })

server.listen(80, () => {
    console.log('\r\n\r\n')
    console.log('signaling server starting...')
    console.log('   example: http://localhost')
    console.log('   signaling: ws://localhost')
    console.log('\r\n\r\n')
})