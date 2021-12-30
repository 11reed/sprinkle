const { exec } = require("child_process");

exec("./target/debug/sprinkle", (error, stdout, stderr) => console.log(stdout));