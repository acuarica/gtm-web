import { GtmService, Commit, CommitsFilter, WorkdirStatusList } from '../../@notes';

async function fetchurl<T>(url: string): Promise<T> {
  return await fetch(url).then(r => r.json())
}

export class WebService implements GtmService {

  fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    return fetchurl(`/data/commits?from=${filter.start}&to=${filter.end}`)
  }

  async fetchProjectList(): Promise<string[]> {
    const value = await fetchurl<string[]>('/data/projects')
    return value.map((p: string) => p.substring(p.lastIndexOf('/') + 1))
  }

  fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return fetchurl('/data/status')
  }

}