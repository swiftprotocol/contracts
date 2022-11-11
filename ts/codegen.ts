import codegen from '@cosmwasm/ts-codegen'

codegen({
  contracts: [
    {
      name: 'Commerce',
      dir: '../contracts/commerce/schema',
    },
    {
      name: 'Trust',
      dir: '../contracts/trust/schema',
    },
  ],
  outPath: './src/',

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: 'index.ts',
      scope: 'contracts',
    },
    types: {
      enabled: true,
    },
    client: {
      enabled: true,
    },
    reactQuery: {
      enabled: false,
      optionalClient: true,
      version: 'v4',
      mutations: true,
      queryKeys: true,
    },
    recoil: {
      enabled: true,
    },
    messageComposer: {
      enabled: true,
    },
  },
}).then(() => {
  console.log('✨ all done!')
})
