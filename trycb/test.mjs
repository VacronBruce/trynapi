import { SimpleCall,  } from './index.js'

const simple = new SimpleCall();
const fp = () => { console.log("Rust call js function");}
simple.registerFn(fp);   
simple.tryFn();
simple.tryFn();
simple.tryFn();
simple.tryFn();
