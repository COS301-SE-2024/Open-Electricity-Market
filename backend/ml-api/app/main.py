import pickle
import os
from pathlib import Path
from contextlib import asynccontextmanager
import psycopg2
import pandas
import sklearn
import numpy
from fastapi.middleware.cors import CORSMiddleware
from fastapi import FastAPI
from pydantic import BaseModel
import dotenv

BASE_DIR = Path(__file__).resolve(strict=True).parent

dotenv.load_dotenv(f"{BASE_DIR}/models/price/.env")
params = {
    "database": os.getenv("DB_DATABASE"),
    "user": os.getenv("DB_USER"),
    "password": os.getenv("DB_PASSWORD"),
    "host": os.getenv("DB_HOST"),
    "port": os.getenv("DB_PORT"),
}
appliance_data_features = [
    "day_hour",
    "day_minute",
    "air_conditioner",
    "air_purifier",
    "boiler",
    "coffee",
    "computer",
    "dehumidifier",
    "dishwasher",
    "dryer",
    "fan",
    "freezer",
    "fridge",
    "internet_router",
    "laptop",
    "micro_wave_oven",
    "phone_charger",
    "printer",
    "printer_3D",
    "radiator",
    "screen",
    "solar_panel",
    "sound_system",
    "tv",
    "vacuum",
    "washing_machine",
]

conn = psycopg2.connect(**params)
cursor_appliance_data = conn.cursor()
cursor_appliance_data.execute(
    """
    WITH
        _air_conditioner_data AS (
            SELECT time, AVG(air_conditioner) AS air_conditioner
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS air_conditioner
                FROM appliance_data
                WHERE appliance = 'air_conditioner'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _air_conditioner_data; /*
        , _air_purifier_data AS (
            SELECT time, AVG(air_purifier) AS air_purifier
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS air_purifier
                FROM appliance_data
                WHERE appliance = 'air_purifier'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _air_purifier_data; /*
        , _boiler_data AS (
            SELECT time, AVG(boiler) AS boiler
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS boiler
                FROM appliance_data
                WHERE appliance = 'boiler'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _boiler_data; /*
        , _coffee_data AS (
            SELECT time, AVG(coffee) AS coffee
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS coffee
                FROM appliance_data
                WHERE appliance = 'coffee'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _coffee_data; /*
        , _computer_data AS (
            SELECT time, AVG(computer) AS computer
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS computer
                FROM appliance_data
                WHERE appliance = 'computer'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _computer_data; /*
        , _dehumidifier_data AS (
            SELECT time, AVG(dehumidifier) AS dehumidifier
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS dehumidifier
                FROM appliance_data
                WHERE appliance = 'dehumidifier'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _dehumidifier_data; /*
        , _dishwasher_data AS (
            SELECT time, AVG(dishwasher) AS dishwasher
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS dishwasher
                FROM appliance_data
                WHERE appliance = 'dishwasher'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _dishwasher_data; /*
        , _dryer_data AS (
            SELECT time, AVG(dryer) AS dryer
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS dryer
                FROM appliance_data
                WHERE appliance = 'dryer'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _dryer_data; /*
        , _fan_data AS (
            SELECT time, AVG(fan) AS fan
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS fan
                FROM appliance_data
                WHERE appliance = 'fan'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _fan_data; /*
        , _freezer_data AS (
            SELECT time, AVG(freezer) AS freezer
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS freezer
                FROM appliance_data
                WHERE appliance = 'freezer'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _freezer_data; /*
        , _fridge_data AS (
            SELECT time, AVG(fridge) AS fridge
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS fridge
                FROM appliance_data
                WHERE appliance = 'fridge'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _fridge_data; /*
        , _internet_router_data AS (
            SELECT time, AVG(internet_router) AS internet_router
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS internet_router
                FROM appliance_data
                WHERE appliance = 'internet_router'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _internet_router_data; /*
        , _laptop_data AS (
            SELECT time, AVG(laptop) AS laptop
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS laptop
                FROM appliance_data
                WHERE appliance = 'laptop'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _laptop_data; /*
        , _micro_wave_oven_data AS (
            SELECT time, AVG(micro_wave_oven) AS micro_wave_oven
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS micro_wave_oven
                FROM appliance_data
                WHERE appliance = 'micro_wave_oven'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _micro_wave_oven_data; /*
        , _phone_charger_data AS (
            SELECT time, AVG(phone_charger) AS phone_charger
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS phone_charger
                FROM appliance_data
                WHERE appliance = 'phone_charger'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _phone_charger_data; /*
        , _printer_data AS (
            SELECT time, AVG(printer) AS printer
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS printer
                FROM appliance_data
                WHERE appliance = 'printer'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _printer_data; /*
        , _printer_3D_data AS (
            SELECT time, AVG(printer_3D) AS printer_3D
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS printer_3D
                FROM appliance_data
                WHERE appliance = 'printer_3D'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _printer_3D_data; /*
        , _radiator_data AS (
            SELECT time, AVG(radiator) AS radiator
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS radiator
                FROM appliance_data
                WHERE appliance = 'radiator'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _radiator_data; /*
        , _screen_data AS (
            SELECT time, AVG(screen) AS screen
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS screen
                FROM appliance_data
                WHERE appliance = 'screen'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _screen_data; /*
        , _solar_panel_data AS (
            SELECT time, AVG(solar_panel) AS solar_panel
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS solar_panel
                FROM appliance_data
                WHERE appliance = 'solar_panel'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _solar_panel_data; /*
        , _sound_system_data AS (
            SELECT time, AVG(sound_system) AS sound_system
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS sound_system
                FROM appliance_data
                WHERE appliance = 'sound_system'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _sound_system_data; /*
        , _tv_data AS (
            SELECT time, AVG(tv) AS tv
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS tv
                FROM appliance_data
                WHERE appliance = 'tv'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _tv_data; /*
        , _vacuum_data AS (
            SELECT time, AVG("vacuum") AS "vacuum"
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS "vacuum"
                FROM appliance_data
                WHERE appliance = 'vacuum'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _vacuum_data; /*
        , _washing_machine_data AS (
            SELECT time, AVG(washing_machine) AS washing_machine
            FROM (
                SELECT time_bucket_gapfill('1 minute', time) AS time, data AS washing_machine
                FROM appliance_data
                WHERE appliance = 'washing_machine'
            )
            GROUP BY time
            ORDER BY time
        ) -- SELECT * FROM _washing_machine_data; /*
        SELECT
            CAST(EXTRACT(HOUR FROM _tv_data.time) AS INTEGER) AS day_hour
            , CAST(EXTRACT(MINUTE FROM _tv_data.time) AS INTEGER) AS day_minute
            , _air_conditioner_data.air_conditioner
            , _air_purifier_data.air_purifier
            , _boiler_data.boiler
            , _coffee_data.coffee
            , _computer_data.computer
            , _dehumidifier_data.dehumidifier
            , _dishwasher_data.dishwasher
            , _dryer_data.dryer
            , _fan_data.fan
            , _freezer_data.freezer
            , _fridge_data.fridge
            , _internet_router_data.internet_router
            , _laptop_data.laptop
            , _micro_wave_oven_data.micro_wave_oven
            , _phone_charger_data.phone_charger
            , _printer_data.printer
            , _printer_3D_data.printer_3D
            , _radiator_data.radiator
            , _screen_data.screen
            , _solar_panel_data.solar_panel
            , _sound_system_data.sound_system
            , _tv_data.tv
            , _vacuum_data."vacuum"
            , _washing_machine_data.washing_machine
        FROM _air_conditioner_data
            JOIN _air_purifier_data ON _air_conditioner_data.time = _air_purifier_data.time
            JOIN _boiler_data ON _air_conditioner_data.time = _boiler_data.time
            JOIN _coffee_data ON _air_conditioner_data.time = _coffee_data.time
            JOIN _computer_data ON _air_conditioner_data.time = _computer_data.time
            JOIN _dehumidifier_data ON _air_conditioner_data.time = _dehumidifier_data.time
            JOIN _dishwasher_data ON _air_conditioner_data.time = _dishwasher_data.time
            JOIN _dryer_data ON _air_conditioner_data.time = _dryer_data.time
            JOIN _fan_data ON _air_conditioner_data.time = _fan_data.time
            JOIN _freezer_data ON _air_conditioner_data.time = _freezer_data.time
            JOIN _fridge_data ON _air_conditioner_data.time = _fridge_data.time
            JOIN _internet_router_data ON _air_conditioner_data.time = _internet_router_data.time
            JOIN _laptop_data ON _air_conditioner_data.time = _laptop_data.time
            JOIN _micro_wave_oven_data ON _air_conditioner_data.time = _micro_wave_oven_data.time
            JOIN _phone_charger_data ON _air_conditioner_data.time = _phone_charger_data.time
            JOIN _printer_data ON _air_conditioner_data.time = _printer_data.time
            JOIN _printer_3D_data ON _air_conditioner_data.time = _printer_3D_data.time
            JOIN _radiator_data ON _air_conditioner_data.time = _radiator_data.time
            JOIN _screen_data ON _air_conditioner_data.time = _screen_data.time
            JOIN _solar_panel_data ON _air_conditioner_data.time = _solar_panel_data.time
            JOIN _sound_system_data ON _air_conditioner_data.time = _sound_system_data.time
            JOIN _tv_data ON _air_conditioner_data.time = _tv_data.time
            JOIN _vacuum_data ON _air_conditioner_data.time = _vacuum_data.time
            JOIN _washing_machine_data ON _air_conditioner_data.time = _washing_machine_data.time
    """
)
appliance_data = pandas.DataFrame(
    cursor_appliance_data.fetchall(), columns=appliance_data_features
)
cursor_appliance_data.close()
with open(f"{BASE_DIR}/models/price/price-model.pkl", "rb") as f:
    price_model: sklearn.pipeline.Pipeline = pickle.load(f)


@asynccontextmanager
async def lifespan(app: FastAPI):
    yield
    conn.close()


app = FastAPI(lifespan=lifespan)
app.add_middleware(
    CORSMiddleware,
    allow_origins=[os.getenv("FRONTEND_URL")],
    allow_credentials=True,
    allow_methods=["POST", "PATCH", "GET", "DELETE"],
    allow_headers=["content_type", "authorization"],
)


class PredictRequest(BaseModel):
    total_impedance: float
    consumer_voltage: float
    transmission_line_voltage: float
    generator_voltage: float
    current_hour: int
    current_minute: int
    time_frame: int


class PricePoint(BaseModel):
    hour: int
    minute: int
    price: float


class PriceData(BaseModel):
    price_list: list[PricePoint]
    best: PricePoint


class PredictResponse(BaseModel):
    data: PriceData
    status: str
    message: str


@app.options("/price_predict")
def price_predict_options():
    return


@app.post("/price_predict", response_model=PredictResponse)
def price_predict(request: PredictRequest):
    cursor = conn.cursor()
    cursor.execute(
        """
        SELECT SUM(2*transaction_fee) FROM transactions
        """
    )
    fee_sum = cursor.fetchall()[0][0]
    cursor.execute(
        """
        SELECT SUM(amount) FROM funds
        """
    )
    funds_total = cursor.fetchall()[0][0]
    cursor.execute(
        """
        SELECT COUNT(*) FROM buy_orders WHERE sought_units > filled_units AND active = TRUE
        """
    )
    num_open_buys = cursor.fetchall()[0][0]
    cursor.execute(
        """
        SELECT COUNT(*) FROM sell_orders WHERE offered_units > claimed_units AND active = TRUE
        """
    )
    num_open_sells = cursor.fetchall()[0][0]
    cursor.close()

    hours = [
        (request.current_hour + x) % 24
        for x in range(request.time_frame)
        for _ in range(60)
    ]
    hours.extend(
        [hours.pop(0) - request.current_hour for _ in range(request.current_hour)]
    )
    minutes = [x for _ in range(request.time_frame) for x in range(60)]
    minutes.extend([minutes.pop(0) for _ in range(request.current_minute)])
    fee_sum_list = [fee_sum for _ in range(request.time_frame * 60)]
    total_funds_list = [funds_total for _ in range(request.time_frame * 60)]
    num_open_buys_list = [num_open_buys for _ in range(request.time_frame * 60)]
    num_open_sells_list = [num_open_sells for _ in range(request.time_frame * 60)]
    total_impedance_list = [
        request.total_impedance for _ in range(request.time_frame * 60)
    ]
    consumer_voltage_list = [
        request.consumer_voltage for _ in range(request.time_frame * 60)
    ]
    tl_voltage_list = [
        request.transmission_line_voltage for _ in range(request.time_frame * 60)
    ]
    generator_voltage_list = [
        request.generator_voltage for _ in range(request.time_frame * 60)
    ]

    df1 = pandas.DataFrame(
        {
            "day_hour": hours,
            "day_minute": minutes,
            "fee_sum": fee_sum_list,
            "funds_total": total_funds_list,
            "total_impedance": total_impedance_list,
            "consumer_voltage": consumer_voltage_list,
            "transmission_line_voltage": tl_voltage_list,
            "generator_voltage": generator_voltage_list,
            "num_open_buys": num_open_buys_list,
            "num_open_sells": num_open_sells_list,
        }
    )

    predict_features = pandas.merge(
        left=df1,
        right=appliance_data,
        how="left",
        left_on=["day_hour", "day_minute"],
        right_on=["day_hour", "day_minute"],
    )

    prediction = price_model.predict(predict_features)
    min_index = numpy.argmin(prediction)
    min_price_hour, min_price_minute = predict_features.iloc[min_index, 0:2]
    min_price = prediction[min_index]
    return {
        "status": "ok",
        "message": "Successfully retrieved price prediction",
        "data": {
            "price_list": [
                {
                    "price": prediction[x],
                    "hour": predict_features.iloc[x, 0],
                    "minute": predict_features.iloc[x, 1],
                }
                for x in range(len(prediction))
            ],
            "best": {
                "hour": min_price_hour,
                "minute": min_price_minute,
                "price": min_price,
            },
        },
    }
