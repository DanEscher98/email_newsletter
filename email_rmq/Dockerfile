FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /source

COPY *.sln .
COPY *.csproj .
RUN dotnet restore -v d

COPY src/ config/ .
RUN dotnet publish -c Release --self-contained -r linux-x64 -f net8.0 \
      -v detailed -o /app --no-restore


FROM mcr.microsoft.com/dotnet/aspnet:8.0 as runtime

ARG EMAIL
ARG EMAIL_PASSWORD
ARG MAIL_NAME
ARG SMTP_DOMAIN
ARG SMTP_PORT

RUN apt-get update && \
    apt-get install -y mailutils && \
    apt install -y postfix

COPY config/main.cf /etc/postfix/main.cf
RUN sh -c 'echo "root: ${EMAIL}" >> /etc/aliases' && \
    sh -c 'echo "${MAIL_NAME}" >> /etc/mailname' && \
    sh -c 'echo "[${SMTP_DOMAIN}]:${SMTP_PORT} ${EMAIL}:${EMAIL_PASSWORD}" >> /etc/postfix/sasl_passwd' && \
    postmap /etc/postfix/sasl_passwd && \
    chmod 0600 /etc/postfix/sasl_passwd /etc/postfix/sasl_passwd.db
CMD service postfix restart

WORKDIR /app
COPY --from=build /app/ .
ENTRYPOINT dotnet rmq_email.dll && tail -f /dev/null
