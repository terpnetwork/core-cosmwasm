import { denom } from '../../configs/chain_config.json'
import Context, { CONTRACT_MAP } from '../setup/context'
import { getClient } from '../utils/client'
import { approveNft, createMinter, mintNft } from '../utils/nft'
import { contracts } from '@terpnetwork/core-cosmwasm-types'
import { ResidualDefault } from '@terpnetwork/core-cosmwasm-types/lib/ResidualRegistry.types'
import { ResidualEntry } from '@terpnetwork/core-cosmwasm-types/lib/ResidualRegistry.types'
import _ from 'lodash'

const { ResidualRegistryClient, ResidualRegistryQueryClient } = contracts.ResidualRegistry

describe('ResidualRegistry', () => {
  const creatorName = 'user1'
  const nonCreatorName = 'user2'
  const recipientName = 'user3'

  let context: Context
  let residualRegistryAddress: string
  let minterAddress: string
  let collectionAddress: string

  beforeAll(async () => {
    context = new Context()
    await context.initializeTestUsers()
    await context.hydrateContext()
    ;[minterAddress, collectionAddress] = await createMinter(context)
    residualRegistryAddress = context.getContractAddress(CONTRACT_MAP.ROYALTY_REGISTRY)
  })

  test('initialize collection residual', async () => {
    const nonCreator = context.testUserMap[nonCreatorName]

    let queryClient = await getClient()
    let contractResponse = await queryClient.getContract(collectionAddress)

    const residualRegistryQueryClient = new ResidualRegistryQueryClient(nonCreator.client, residualRegistryAddress)
    const residualRegistryClient = new ResidualRegistryClient(
      nonCreator.client,
      nonCreator.address,
      residualRegistryAddress,
    )

    let residualDefault = await residualRegistryQueryClient.collectionResidualDefault({ collection: collectionAddress })

    if (residualDefault) {
      // Already initialized in prior test run
    } else {
      let response = await residualRegistryClient.initializeCollectionResidual(
        { collection: collectionAddress },
        'auto',
        'initialize-collection-residual',
      )
      expect(response).toBeTruthy()

      residualDefault = await residualRegistryQueryClient.collectionResidualDefault({ collection: collectionAddress })
      expect(residualDefault).toBeTruthy()
      expect(residualDefault?.collection).toEqual(collectionAddress)
      expect(residualDefault?.residual_entry.recipient).toBeTruthy()
      expect(residualDefault?.residual_entry.share).toBeTruthy()
      expect(residualDefault?.residual_entry.updated).toBeNull()
    }
  })

  test('update collection residual default', async () => {
    const creator = context.testUserMap[creatorName]
    const recipient = context.testUserMap[recipientName]

    const residualRegistryQueryClient = new ResidualRegistryQueryClient(creator.client, residualRegistryAddress)
    const residualRegistryClient = new ResidualRegistryClient(creator.client, creator.address, residualRegistryAddress)

    let residualDefaultBefore = (await residualRegistryQueryClient.collectionResidualDefault({
      collection: collectionAddress,
    })) as ResidualDefault
    expect(residualDefaultBefore).toBeTruthy()
    expect(residualDefaultBefore.collection).toEqual(collectionAddress)

    let shareDelta = '0.005'
    let response = await residualRegistryClient.updateCollectionResidualDefault(
      { collection: collectionAddress, decrement: false, recipient: recipient.address, shareDelta },
      'auto',
      'update-collection-residual-default',
    )
    expect(response).toBeTruthy()

    let residualDefaultAfter = (await residualRegistryQueryClient.collectionResidualDefault({
      collection: collectionAddress,
    })) as ResidualDefault
    expect(residualDefaultAfter).toBeTruthy()
    expect(residualDefaultAfter.collection).toEqual(collectionAddress)
    expect(residualDefaultAfter.residual_entry.recipient).toEqual(recipient.address)
    expect(parseFloat(residualDefaultBefore.residual_entry.share) + parseFloat(shareDelta)).toEqual(
      parseFloat(residualDefaultAfter.residual_entry.share),
    )
    expect(residualDefaultAfter.residual_entry.updated).toBeTruthy()
  })

  test('set collection residual protocol', async () => {
    const creator = context.testUserMap[creatorName]
    const recipient = context.testUserMap[recipientName]

    // Dummy protocol address just for testing purposes
    const protocolAddress = 'terp1tu06v6gzcc0gugfrmxrfrj20f9y499ccsjp8fy'

    const residualRegistryQueryClient = new ResidualRegistryQueryClient(creator.client, residualRegistryAddress)
    const residualRegistryClient = new ResidualRegistryClient(creator.client, creator.address, residualRegistryAddress)

    let residualDefaultBefore = (await residualRegistryQueryClient.collectionResidualDefault({
      collection: collectionAddress,
    })) as ResidualDefault
    expect(residualDefaultBefore).toBeTruthy()
    expect(residualDefaultBefore.collection).toEqual(collectionAddress)

    let share = '0.07'
    let response = await residualRegistryClient.setCollectionResidualProtocol(
      { collection: collectionAddress, protocol: protocolAddress, recipient: recipient.address, share },
      'auto',
      'set-collection-residual-protocol',
    )
    expect(response).toBeTruthy()

    let residualProtocol = await residualRegistryQueryClient.collectionResidualProtocol({
      collection: collectionAddress,
      protocol: protocolAddress,
    })
    expect(residualProtocol).toBeTruthy()
    expect(residualProtocol?.residual_entry.recipient).toEqual(recipient.address)
    expect(residualProtocol?.residual_entry.share).toEqual(share)
    expect(residualProtocol?.residual_entry.updated).toBeTruthy()
  })
})
