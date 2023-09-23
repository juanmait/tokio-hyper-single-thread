#!/usr/bin/env node
const http = require("node:http");
const util = require("node:util");

const args = util.parseArgs({
	args: process.argv.slice(2),
	tokens: true,
	options: { port: { type: "string", short: "p", default: "3001" } },
});

let count = 0;

const server = http.createServer((_req, res) => {
	count++;
	res.end(`home! counter = ${count}`);
});

server.listen(3001, "127.0.0.1", () => {
	console.log("NodeJS HTTP Server listening at port", args.values.port);
});
