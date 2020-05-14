import { GtmService, Commit, CommitsFilter, WorkdirStatusList } from '../../@notes';

export class WebService implements GtmService {

  constructor(readonly host: string = '') { }

  fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    return this.fetchurl(`/data/commits?from=${filter.start}&to=${filter.end}`)
  }

  async fetchProjectList(): Promise<string[]> {
    const value = await this.fetchurl<string[]>('/data/projects')
    return value.map((p: string) => p.substring(p.lastIndexOf('/') + 1))
  }

  fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return this.fetchurl('/data/status')
  }

  private async fetchurl<T>(url: string): Promise<T> {
    return await fetch(this.host + url).then(r => r.json())
  }

}