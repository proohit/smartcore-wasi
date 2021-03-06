const http = require("http");

const hostname = "0.0.0.0";
const port = 3001;
let loadModel;

const lib = "./lib/js/glue.js";

// Might need it for reloading modules
// function resetLib() {
//   try {
//     console.log("reseting fn");
//     loadModel = undefined;
//     delete require.cache[require.resolve(lib)];
//     require(lib).initialize();
//   } catch (e) {
//     console.log(e);
//   }
// }

const server = http.createServer((req, res) => {
  if (!loadModel) {
    loadModel = require(lib).loadModel;
  }
  loadModel().then((pred) => {
    res.end(pred);
  });
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
