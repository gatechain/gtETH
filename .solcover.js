module.exports = {
    configureYulOptimizer: true,
    mocha: {
        grep: "@skip-on-coverage", // Find everything with this tag
        invert: true               // Run the grep's inverse set.
    },
    skipFiles: [
        'test',
        'interface',
        'UniswapBatchQuery.sol',
        'GateioTokenVerify.sol',
        'AggregationExecutor.sol',
        'CallbackValidation.sol',
        'base/AggregationExecutorBase.sol',
    ]
};