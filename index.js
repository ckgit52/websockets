let ws = new WebSocket('ws://localhost:8080/tokens/ws');
ws.onopen = function() {
    console.log('Connected to WebSocket server');
    ws.send('Hello, server!');
};
ws.onmessage = function(event) {
    console.log('Server says: ' + event.data);
};
