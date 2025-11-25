#pragma once

#include <stdint.h>
#include <stdbool.h>

typedef uint64_t SteamID_t;
typedef uint32_t AppId_t;
typedef uint32_t RTime32;

#define k_nSteamEncryptedAppTicketSymmetricKeyLen 32

#ifdef __cplusplus
extern "C"
{
#endif

	bool SteamEncryptedAppTicket_BDecryptTicket(const uint8_t *rgubTicketEncrypted, uint32_t cubTicketEncrypted,
												uint8_t *rgubTicketDecrypted, uint32_t *pcubTicketDecrypted,
												const uint8_t rgubKey[k_nSteamEncryptedAppTicketSymmetricKeyLen], int cubKey);

	bool SteamEncryptedAppTicket_BIsTicketForApp(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted, AppId_t nAppID);

	RTime32 SteamEncryptedAppTicket_GetTicketIssueTime(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted);

	void SteamEncryptedAppTicket_GetTicketSteamID(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted, SteamID_t *psteamID);

	AppId_t SteamEncryptedAppTicket_GetTicketAppID(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted);

	bool SteamEncryptedAppTicket_BUserOwnsAppInTicket(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted, AppId_t nAppID);

	bool SteamEncryptedAppTicket_BUserIsVacBanned(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted);

	bool SteamEncryptedAppTicket_BGetAppDefinedValue(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted, uint32_t *pValue);

	const uint8_t *SteamEncryptedAppTicket_GetUserVariableData(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted, uint32_t *pcubUserData);

	bool SteamEncryptedAppTicket_BIsTicketSigned(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted, const uint8_t *pubRSAKey, uint32_t cubRSAKey);

	bool SteamEncryptedAppTicket_BIsLicenseBorrowed(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted);

	bool SteamEncryptedAppTicket_BIsLicenseTemporary(uint8_t *rgubTicketDecrypted, uint32_t cubTicketDecrypted);

#ifdef __cplusplus
}
#endif