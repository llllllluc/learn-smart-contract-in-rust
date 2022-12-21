import type { Env } from "@terra-money/terrain";
import { TokenFactoryClient } from './clients/TokenFactoryClient';

export class Lib extends TokenFactoryClient {
  env: Env;

  constructor(env: Env) {
    super(env.client, env.defaultWallet, env.refs['token_factory'].contractAddresses.default);
    this.env = env;
  }
};

export default Lib;
