import { GtmService, Commit, CommitsFilter, WorkdirStatusList } from '@gtm/notes';

export class WebService implements GtmService {

  constructor(readonly host: string = '') { }

  getVersion(): Promise<string | null> {
    return this.fetchurl('/version')
  }

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

export class DelayedService implements GtmService {

  constructor(readonly service: GtmService, readonly timeout: number) { }

  getVersion(): Promise<string | null> {
    return this.service.getVersion()
  }

  async fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    return this.delay(async () => this.service.fetchCommits(filter))
  }

  async fetchProjectList(): Promise<string[]> {
    return this.delay(async () => this.service.fetchProjectList())
  }

  async fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return this.delay(async () => this.service.fetchWorkdirStatus())
  }

  private delay<T>(action: () => T, timeout: number = this.timeout): Promise<T> {
    return new Promise(function (resolve) {
      setTimeout(() => {
        resolve(action())
      }, timeout)
    })
  }

}
export class FailureService implements GtmService {

  constructor(readonly service: GtmService) { }

  getVersion(): Promise<string | null> {
    throw new Error('Could not retrieve version');
  }

  async fetchCommits(filter: CommitsFilter): Promise<Commit[]> {
    return this.fail(async () => this.service.fetchCommits(filter))
  }

  async fetchProjectList(): Promise<string[]> {
    return this.fail(async () => this.service.fetchProjectList())
  }

  async fetchWorkdirStatus(): Promise<WorkdirStatusList> {
    return this.fail(async () => this.service.fetchWorkdirStatus())
  }

  private fail<T>(action: () => T): Promise<T> {
    return new Promise(function (_, reject) {
      reject(action())
    })
  }

}