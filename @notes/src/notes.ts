import moment from 'moment'
import { pad0 } from './format.js'

///
export type Seconds = number

///
export class FileNote {
  TimeSpent: Seconds = 0;
  readonly Timeline: { [id: string]: Seconds } = {};
  Status = '';
  constructor(readonly SourceFile: string, timeSpent: Seconds) {
    this.TimeSpent = timeSpent;
  }
}

///
export type WorkdirStatus = {
  Total: Seconds;
  Label: string;
  CommitNote: { Files: FileNote[] };
}

///
export type WorkdirStatusList = { [projectName: string]: WorkdirStatus }

/// 
export class Commit {
  Author = '';
  Date = '';
  When = '';
  Hash = '';
  Subject = '';
  Message = '';
  timeSpent?: Seconds;
  constructor(readonly Project: string, readonly Note: { Files: FileNote[] }, timeSpent: Seconds) {
    this.timeSpent = timeSpent
  }
}

export class Project {
  total = 0;
  commits: Commit[] = [];
  files: { [fileName: string]: FileNote } = {};
  status: FileStatus<number> = { 'm': 0, 'r': 0, 'd': 0 };
  timeline: {
    [id: string]: {
      [hour: number]: {
        total: number;
      };
    };
  } = {};
  timelineMatrix: {
    x: string;
    y: string;
    v: number;
  }[] = [];
  constructor(readonly name: string) { }
}

///
export type ProjectList = { [id: string]: Project }

///
export type FileStatus<T> = { [s: string]: T }

/// Hours is expressed by the total field in seconds.
export type DailyHours = { [date: string]: { total: number } }

///
export type Stats = {
  projects: ProjectList;
  totalSecs: Seconds;
  status: FileStatus<number>;
}

///
export function computeStats(commits: Commit[]): Stats {
  const projects: ProjectList = {};
  const status: FileStatus<number> = { 'm': 0, 'r': 0, 'd': 0 };
  let totalSecs: Seconds = 0;

  for (const commit of commits) {
    let project = projects[commit.Project];
    if (project === undefined) {
      project = new Project(commit.Project);
      projects[commit.Project] = project;
    }
    project.commits.push(commit);
    if (commit.Note.Files === null) {
      console.warn('gtm check: Commit note files not available:', commit);
      continue;
    }
    let commitTimeSpent = 0;
    for (const file of commit.Note.Files) {
      commitTimeSpent += file.TimeSpent

      let fileSecs = 0;
      for (const timestamp2 in file.Timeline) {
        const timestamp = Number(timestamp2)
        const secs = file.Timeline[timestamp];
        if (secs > 3600) console.warn('gtm check: Duration (in seconds) should be less than 3600:', secs);
        if (timestamp % 3600 !== 0) console.warn('gtm check: Timestamp (unix time) should be by the hour:', timestamp);
        project.total += secs;
        const date = moment.unix(timestamp).startOf('day').format('YYYY-MM-DD');
        const hour = moment.unix(timestamp).hour();
        let dateline = project.timeline[date];
        if (dateline === undefined) {
          dateline = {};
          project.timeline[date] = dateline;
        }
        let hourline = dateline[hour];
        if (hourline === undefined) {
          hourline = { total: 0 };
          dateline[hour] = hourline;
        }
        hourline.total += secs;
        fileSecs += secs;
        totalSecs += secs
        // console.assert(Object.keys(status).includes(file.Status), `Unexpected status '${file.Status}' for file ${file.SourceFile}`)
        status[file.Status] += secs
        project.status[file.Status] += secs
      }
      if (fileSecs !== file.TimeSpent) console.warn('gtm check: Timeline seconds does not add up to duration in file.');

      const fileNote = project.files[file.SourceFile]
      if (!fileNote) {
        project.files[file.SourceFile] = new FileNote(file.SourceFile, file.TimeSpent)
      } else {
        fileNote.TimeSpent += file.TimeSpent
      }
    }

    commit.timeSpent = commitTimeSpent
  }

  return { projects, totalSecs, status }
}

///
export function getDaily(projects: ProjectList): DailyHours {
  const daily: DailyHours = {};
  for (const pkey in projects) {
    const p = projects[pkey]
    const data = [];
    for (const date in p.timeline) {
      for (const h in p.timeline[date]) {
        const hour = Number(h)
        const secs = p.timeline[date][hour];
        data.push({
          x: `${pad0(hour)}:00`, y: date, v: secs.total,
        });
        let day = daily[date];
        if (day === undefined) {
          day = { total: 0 };
          daily[date] = day;
        }
        day.total += secs.total;
      }
    }
    p.timelineMatrix = data;
  }

  return daily
}

export function computeWorkdirStatus(workdirStatus: WorkdirStatusList): Stats {
  const commits: Commit[] = []
  for (const p in workdirStatus) {
    const cn = workdirStatus[p]
    commits.push(new Commit(p, cn.CommitNote, 0))
  }

  return computeStats(commits)
}