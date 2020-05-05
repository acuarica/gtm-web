import moment from "moment"
import { pad0 } from "@gtm/format"

///
export function computeStats(commits) {
  const projects = [];
  const status = {'m': 0, 'r': 0, 'd': 0};
  let totalSecs = 0;

  for (const commit of commits) {
    let project = projects[commit.Project];
    if (project === undefined) {
      project = { name: commit.Project, total: 0, commits: [], timeline: {}, timelineMatrix: [] };
      projects[commit.Project] = project;
    }
    project.commits.push(commit);
    if (commit.Note.Files === null) {
      console.warn("gtm check: Commit note files not available:", commit);
      continue;
    }
    let commitTimeSpent = 0;
    for (const file of commit.Note.Files) {
      commitTimeSpent += file.TimeSpent

      let fileSecs = 0;
      for (const timestamp2 in file.Timeline) {
        const timestamp = Number(timestamp2)
        const secs = file.Timeline[timestamp];
        if (secs > 3600) console.warn("gtm check: Duration (in seconds) should be less than 3600:", secs);
        if (timestamp % 3600 !== 0) console.warn("gtm check: Timestamp (unix time) should be by the hour:", timestamp);
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
      }
      if (fileSecs !== file.TimeSpent) console.warn("gtm check: Timeline seconds does not add up to duration in file.");
    }

    commit.timeSpent = commitTimeSpent
  }

  return { projects, totalSecs, status }
}

///
export function getDaily(projects) {
  const daily = {};
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

export function computeWorkdirStatus(wds) {
  const commits = []
  for (const p in wds) {
    const cn = wds[p]
    commits.push({ Project: p, Note: cn.CommitNote, timeSpent: 0 })
  }

  return computeStats(commits)
}