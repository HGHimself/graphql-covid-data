export const getPopulationFromRate = (d) => {
  d.TOTALPOPCASE = (d.CASECOUNT * 100000) / d.CASERATE;
  d.TOTALPOPHOSP = (d.HOSPITALIZEDCOUNT * 100000) / d.HOSPITALIZEDRATE;
  d.TOTALPOPDEATH = (d.DEATHCOUNT * 100000) / d.DEATHRATE;
  return d;
};
