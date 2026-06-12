Pebble.addEventListener('ready', () => {
    PebbleTS.sendAppMessage({
        App_ExampleKey: 'Hello from TypeScript!',
    })
})
