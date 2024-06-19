import { defineLoader } from 'vitepress'
// @ts-ignore
import { Octokit } from '@octokit/rest'
// @ts-ignore
import type { GetResponseDataTypeFromEndpointMethod } from '@octokit/types'
// @ts-ignore
import fetch from "node-fetch";

const octokit = new Octokit({
    request: {
        fetch: fetch
    }
})

type GitHubReleaseList = GetResponseDataTypeFromEndpointMethod<typeof octokit.repos.listReleases>

declare const data: GitHubReleaseList
export { data }

export default defineLoader({
  async load(): Promise<GitHubReleaseList> {
    const releases = await octokit.paginate(octokit.repos.listReleases, {
      owner: 'sleeprite',
      repo: 'rudis',
      per_page: 100,
    })
    return releases
  },
})