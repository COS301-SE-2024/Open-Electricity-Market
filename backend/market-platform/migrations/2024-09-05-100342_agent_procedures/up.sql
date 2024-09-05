-- Your SQL goes here
CREATE FUNCTION sample_appliance(float8,text) RETURNS float8 AS $$
select average from (select Avg(data) as average,abs(EXTRACT(Second FROM time) - $1) as dist from appliance_data where appliance = $2
Group by dist
ORDER BY dist
LIMIT 1);
$$ LANGUAGE sql;


CREATE FUNCTION total_appliance(text) RETURNS float8 AS $$
select SUM(data) as total from appliance_data where appliance = $1;
$$ LANGUAGE sql;

CREATE FUNCTION appliance_curve(text[]) RETURNS TABLE (
        appliance text,
        data float8 ) AS $$
select appliance,data from (SELECT appliance,time_bucket('1 hour', time) AS bucket,
  avg(data) AS data
FROM appliance_data
WHERE appliance = ANY ('{"printer","printer_3D"}'::text[])
GROUP BY appliance,bucket
ORDER BY appliance,bucket ASC);
$$ LANGUAGE sql;

