const DATA_STREAM_PUBLIC_TOKEN = "{{ token }}";
const APPLY_CHECKBOX_KEY = "__APPLY_CHECKBOX";
const TIMESTAMP_KEY = "__TIMESTAMP";

/**
 * Handling code for receiving data from the DSU native application.
 *
 * Receives data in the following format:
 * {
 *   "__TIMESTAMP": 923498324, // ms since epoch
 *   "subcompName": {
 *     "__APPLY_CHECKBOX": "Control Node Name", // name of checkbox to use to determine whether this subcomp should be applied
 *     "Control Node 1": "Control Node value",
 *     // ...
 *   },
 *   // ...
 * }
 */

(function () {
  let dataStream;
  let lastReceived = 0;

  return {
    init: function (comp, context) {
      console.log("Composition script opened:", comp.name);
      dataStream = context.utils.createDataStream(
        DATA_STREAM_PUBLIC_TOKEN,
        (status, data) => {
          if (status === "message") {
            const { [TIMESTAMP_KEY]: timestamp, ...payload } = data.payload;
            if (timestamp < lastReceived) {
              console.warn(
                "Received an outdated message, skipping.",
                timestamp,
                lastReceived
              );
              return;
            }
            lastReceived = timestamp;
            console.info("Latency:", new Date().getTime() - data.ts);
            for (const [key, value] of Object.entries(payload)) {
              const subComp = comp.find(key)[0];
              if (subComp) {
                let shouldUpdate = true;
                const currentPayload = subComp.getPayload2();
                if (Object.hasOwnProperty(value, APPLY_CHECKBOX_KEY)) {
                  if (
                    Object.hasOwnProperty(
                      currentPayload,
                      value[APPLY_CHECKBOX_KEY]
                    )
                  ) {
                    shouldUpdate = currentPayload[value[APPLY_CHECKBOX_KEY]];
                  } else {
                    console.error(
                      "the given toggle control node",
                      value[APPLY_CHECKBOX_KEY],
                      "was not found in",
                      subComp
                    );
                  }
                }

                if (shouldUpdate) {
                  subComp.setPayload(value);
                }
              } else {
                console.error(
                  "couldn't find",
                  subComp,
                  "in the composition (it was provided by the data stream)"
                );
              }
            }
          }
        }
      );
    },

    close: function (comp, context) {
      console.log("Composition script closed:", comp.name);
      dataStream.close();
    },
  };
})();
