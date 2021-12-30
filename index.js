const { exec } = require("child_process");
// const sprinkleCss = require("./css/sprinkle.css");

exec("./target/debug/sprinkle", (error, stdout, stderr) => console.log(stdout));