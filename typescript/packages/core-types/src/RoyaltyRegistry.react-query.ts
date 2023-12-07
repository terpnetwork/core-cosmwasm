/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.30.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery } from "react-query";
import { Decimal, InstantiateMsg, Config, ExecuteMsg, QueryMsg, QueryBoundForString, QueryOptionsForString, NullableResidualDefault, Addr, Timestamp, Uint64, ResidualDefault, ResidualEntry, NullableResidualProtocol, ResidualProtocol, ResidualPaymentResponse, ArrayOfResidualProtocol } from "./ResidualRegistry.types";
import { ResidualRegistryQueryClient } from "./ResidualRegistry.client";
export const residualRegistryQueryKeys = {
  contract: ([{
    contract: "residualRegistry"
  }] as const),
  address: (contractAddress: string | undefined) => ([{ ...residualRegistryQueryKeys.contract[0],
    address: contractAddress
  }] as const),
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...residualRegistryQueryKeys.address(contractAddress)[0],
    method: "config",
    args
  }] as const),
  collectionResidualDefault: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...residualRegistryQueryKeys.address(contractAddress)[0],
    method: "collection_residual_default",
    args
  }] as const),
  collectionResidualProtocol: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...residualRegistryQueryKeys.address(contractAddress)[0],
    method: "collection_residual_protocol",
    args
  }] as const),
  residualProtocolByCollection: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...residualRegistryQueryKeys.address(contractAddress)[0],
    method: "residual_protocol_by_collection",
    args
  }] as const),
  residualPayment: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...residualRegistryQueryKeys.address(contractAddress)[0],
    method: "residual_payment",
    args
  }] as const)
};
export const residualRegistryQueries = {
  config: <TData = Config,>({
    client,
    options
  }: ResidualRegistryConfigQuery<TData>): UseQueryOptions<Config, Error, TData> => ({
    queryKey: residualRegistryQueryKeys.config(client?.contractAddress),
    queryFn: () => client ? client.config() : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  collectionResidualDefault: <TData = NullableResidualDefault,>({
    client,
    args,
    options
  }: ResidualRegistryCollectionResidualDefaultQuery<TData>): UseQueryOptions<NullableResidualDefault, Error, TData> => ({
    queryKey: residualRegistryQueryKeys.collectionResidualDefault(client?.contractAddress, args),
    queryFn: () => client ? client.collectionResidualDefault({
      collection: args.collection
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  collectionResidualProtocol: <TData = NullableResidualProtocol,>({
    client,
    args,
    options
  }: ResidualRegistryCollectionResidualProtocolQuery<TData>): UseQueryOptions<NullableResidualProtocol, Error, TData> => ({
    queryKey: residualRegistryQueryKeys.collectionResidualProtocol(client?.contractAddress, args),
    queryFn: () => client ? client.collectionResidualProtocol({
      collection: args.collection,
      protocol: args.protocol
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  residualProtocolByCollection: <TData = ArrayOfResidualProtocol,>({
    client,
    args,
    options
  }: ResidualRegistryResidualProtocolByCollectionQuery<TData>): UseQueryOptions<ArrayOfResidualProtocol, Error, TData> => ({
    queryKey: residualRegistryQueryKeys.residualProtocolByCollection(client?.contractAddress, args),
    queryFn: () => client ? client.residualProtocolByCollection({
      collection: args.collection,
      queryOptions: args.queryOptions
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  residualPayment: <TData = ResidualPaymentResponse,>({
    client,
    args,
    options
  }: ResidualRegistryResidualPaymentQuery<TData>): UseQueryOptions<ResidualPaymentResponse, Error, TData> => ({
    queryKey: residualRegistryQueryKeys.residualPayment(client?.contractAddress, args),
    queryFn: () => client ? client.residualPayment({
      collection: args.collection,
      protocol: args.protocol
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  })
};
export interface ResidualRegistryReactQuery<TResponse, TData = TResponse> {
  client: ResidualRegistryQueryClient | undefined;
  options?: UseQueryOptions<TResponse, Error, TData>;
}
export interface ResidualRegistryResidualPaymentQuery<TData> extends ResidualRegistryReactQuery<ResidualPaymentResponse, TData> {
  args: {
    collection: string;
    protocol?: string;
  };
}
export function useResidualRegistryResidualPaymentQuery<TData = ResidualPaymentResponse>({
  client,
  args,
  options
}: ResidualRegistryResidualPaymentQuery<TData>) {
  return useQuery<ResidualPaymentResponse, Error, TData>(residualRegistryQueryKeys.residualPayment(client?.contractAddress, args), () => client ? client.residualPayment({
    collection: args.collection,
    protocol: args.protocol
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ResidualRegistryResidualProtocolByCollectionQuery<TData> extends ResidualRegistryReactQuery<ArrayOfResidualProtocol, TData> {
  args: {
    collection: string;
    queryOptions?: QueryOptionsForString;
  };
}
export function useResidualRegistryResidualProtocolByCollectionQuery<TData = ArrayOfResidualProtocol>({
  client,
  args,
  options
}: ResidualRegistryResidualProtocolByCollectionQuery<TData>) {
  return useQuery<ArrayOfResidualProtocol, Error, TData>(residualRegistryQueryKeys.residualProtocolByCollection(client?.contractAddress, args), () => client ? client.residualProtocolByCollection({
    collection: args.collection,
    queryOptions: args.queryOptions
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ResidualRegistryCollectionResidualProtocolQuery<TData> extends ResidualRegistryReactQuery<NullableResidualProtocol, TData> {
  args: {
    collection: string;
    protocol: string;
  };
}
export function useResidualRegistryCollectionResidualProtocolQuery<TData = NullableResidualProtocol>({
  client,
  args,
  options
}: ResidualRegistryCollectionResidualProtocolQuery<TData>) {
  return useQuery<NullableResidualProtocol, Error, TData>(residualRegistryQueryKeys.collectionResidualProtocol(client?.contractAddress, args), () => client ? client.collectionResidualProtocol({
    collection: args.collection,
    protocol: args.protocol
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ResidualRegistryCollectionResidualDefaultQuery<TData> extends ResidualRegistryReactQuery<NullableResidualDefault, TData> {
  args: {
    collection: string;
  };
}
export function useResidualRegistryCollectionResidualDefaultQuery<TData = NullableResidualDefault>({
  client,
  args,
  options
}: ResidualRegistryCollectionResidualDefaultQuery<TData>) {
  return useQuery<NullableResidualDefault, Error, TData>(residualRegistryQueryKeys.collectionResidualDefault(client?.contractAddress, args), () => client ? client.collectionResidualDefault({
    collection: args.collection
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ResidualRegistryConfigQuery<TData> extends ResidualRegistryReactQuery<Config, TData> {}
export function useResidualRegistryConfigQuery<TData = Config>({
  client,
  options
}: ResidualRegistryConfigQuery<TData>) {
  return useQuery<Config, Error, TData>(residualRegistryQueryKeys.config(client?.contractAddress), () => client ? client.config() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}