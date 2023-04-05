import http from 'http';
import fs from 'fs';

const PORT = 9000;

function handleRequest(request, response) {
  console.log('request', request.url);

  response.setHeader('Access-Control-Allow-Origin', '*');
  response.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  response.setHeader('Access-Control-Allow-Headers', 'Content-Type, Access-Control-Allow-Headers, Authorization, X-Requested-With');

  // send favicon.ico
  if (request.url === '/favicon.ico') {
    response.setHeader('Content-Type', 'image/x-icon');
    try {
      const favicon = fs.readFileSync('./favicon.ico');
      response.end(favicon);
    } catch (err) {
      console.error(err);
      response.statusCode = 500;
      response.end('Internal Server Error');
    }
  } else
 
  if (request.url === '/pkg/gfx_bg.wasm') {
    response.setHeader('Content-Type', 'application/wasm');
    try {
      const wasm = fs.readFileSync('../pkg/gfx_bg.wasm');
      response.end(wasm);
    } catch (err) {
      console.error(err);
      response.statusCode = 500;
      response.end('Internal Server Error');
    }
  } else if (request.url === '/pkg/gfx.js' || request.url === '/pkg/gfx_bg.js') {
    response.setHeader('Content-Type', 'text/javascript');
    try {
      const js = fs.readFileSync('..' + request.url);
      response.end(js);
    } catch (err) {
      console.error(err);
      response.statusCode = 500;
      response.end('Internal Server Error');
    }
  } else if (request.url === '/index.html') {
    response.setHeader('Content-Type', 'text/html');
    try {
      const html = fs.readFileSync('./index.html');
      response.end(html);
    } catch (err) {
      console.error(err);
      response.statusCode = 500;
      response.end('Internal Server Error');
    }
  } else {
    response.statusCode = 404;
    response.end('Not Found');
  }


  request.on('data', (data) => {
    console.log('data', data);
  });
}

const server = http.createServer(handleRequest);

server.listen(PORT, () => {
  console.log(`Server listening on: http://localhost:${PORT}`);
});
