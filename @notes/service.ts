import { Commit, WorkdirStatusList } from './notes';

export class GtmErr {

  constructor(readonly reason: string, readonly exitCode?: number) { }

}

export interface CommitsFilter {
  start: string;
  end: string;
  message?: string;
}
export interface GtmService {

  getVersion(): Promise<string | null>;

  fetchCommits(filter: CommitsFilter): Promise<Commit[]>;

  fetchProjectList(): Promise<string[]>;

  fetchWorkdirStatus(): Promise<WorkdirStatusList>;

}