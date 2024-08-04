const DATA_STREAM_PUBLIC_TOKEN = "{{ token }}";

(function () {
  let dataStream;

  return {
    init: function (comp, context) {
      console.log("Composition script opened:", comp.name);
      dataStream = context.utils.createDataStream(
        DATA_STREAM_PUBLIC_TOKEN,
        (status, data) => {
          if (status === "message") {
            console.info("Latency:", new Date().getTime() - data.ts);
            for (const [key, value] of Object.entries(data.payload)) {
              const subComp = comp.find(key)[0];
              if (subComp) {
                subComp.setPayload(value);
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

    close: function (comp, _context) {
      console.log("Composition script closed:", comp.name);
      dataStream.close();
    },
  };
})();
