import { SimpleCall } from './index.js'

const simple = new SimpleCall();
const fp = () => { console.log("Rust call js function");}
simple.registerCb(fp);
simple.tryCb();
simple.tryCb(); //Second time execution will pop up error