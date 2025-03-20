import fs from "fs/promises";

const randomNumber = Math.round(Math.random() * 100 + 1);
console.log({ randomNumber });

// test30
await fs.writeFile(`test_${randomNumber}.js`, "test");
