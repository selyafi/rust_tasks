// @generated automatically by Diesel CLI.

diesel::table! {
    AdditionalContactsSchedule (Id) {
        Id -> Uuid,
        AdditionalContactId -> Nullable<Uuid>,
        OpenTime -> Nullable<Time>,
        CloseTime -> Nullable<Time>,
        DayOfWeek -> Int4,
        IsClosed -> Bool,
        Is24Hours -> Bool,
        BreakStart -> Nullable<Time>,
        BreakEnd -> Nullable<Time>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    AdditionalSettings (Id) {
        Id -> Uuid,
        AddressType -> Int2,
        #[max_length = 256]
        CustomAddress -> Nullable<Varchar>,
        SendNotificationToPassengerByEmail -> Bool,
        SendNotificationToPassengerBySms -> Bool,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        CaseId -> Uuid,
        ArrivalAirportId -> Nullable<Uuid>,
    }
}

diesel::table! {
    Addresses (ID) {
        ID -> Uuid,
        #[max_length = 5]
        RF -> Nullable<Varchar>,
        #[max_length = 5]
        ICAO -> Nullable<Varchar>,
        #[max_length = 512]
        NameLat -> Nullable<Varchar>,
        #[max_length = 512]
        Name -> Nullable<Varchar>,
        #[max_length = 256]
        Email -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    AirportTranslation (ID) {
        ID -> Uuid,
        AirportId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    Airports (ID) {
        ID -> Uuid,
        #[max_length = 5]
        IATA -> Nullable<Varchar>,
        #[max_length = 5]
        ICAO -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ArchivePolicySettings (Id) {
        Id -> Uuid,
        Type -> Nullable<Int4>,
        Period -> Int4,
        CreatedAt -> Timestamptz,
        CreatedBy -> Uuid,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    BaggageContentCategories (Id) {
        Id -> Int4,
        #[max_length = 50]
        RU -> Varchar,
        #[max_length = 50]
        EN -> Varchar,
    }
}

diesel::table! {
    BaggageContents (Id) {
        Id -> Uuid,
        BaggagePlaceId -> Nullable<Uuid>,
        BaggageCategoryId -> Nullable<Int4>,
        #[max_length = 64]
        BaggageObject -> Nullable<Varchar>,
        #[max_length = 128]
        BaggageObjectDescription -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    BaggageOwners (Id) {
        Id -> Uuid,
        BaggagePlaceId -> Uuid,
        #[max_length = 32]
        Surname -> Nullable<Varchar>,
        #[max_length = 32]
        Name -> Nullable<Varchar>,
        #[max_length = 32]
        SecondName -> Nullable<Varchar>,
        #[max_length = 16]
        Initials -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    BaggagePhotos (Id) {
        Id -> Uuid,
        BaggagePlaceId -> Nullable<Uuid>,
        #[max_length = 128]
        MimeType -> Nullable<Varchar>,
        Size -> Int4,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        PhotoViewType -> Nullable<Int2>,
    }
}

diesel::table! {
    BaggagePlaces (Id) {
        Id -> Uuid,
        CaseId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Status -> Int2,
        PlaceNumber -> Int2,
        LostInAirportId -> Nullable<Uuid>,
        LostReasonId -> Nullable<Uuid>,
        DeliveredAt -> Nullable<Timestamptz>,
        #[max_length = 512]
        StorageLocation -> Nullable<Varchar>,
        BaggageIssueType -> Nullable<Int2>,
        DeliveryCost -> Nullable<Numeric>,
        DeliveryCostCurrencyId -> Nullable<Uuid>,
    }
}

diesel::table! {
    BaggageProperties (Id) {
        Id -> Uuid,
        BaggagePlaceId -> Uuid,
        ColorId -> Uuid,
        MaterialId -> Uuid,
        #[max_length = 16]
        BaggageTagNumber -> Nullable<Varchar>,
        #[max_length = 8]
        BaggageWeight -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        BaggageTypeId -> Uuid,
        #[max_length = 16]
        DeliveryTagNumber -> Nullable<Varchar>,
        #[max_length = 512]
        AdditionalSpecifics -> Nullable<Varchar>,
        #[max_length = 512]
        BaggageTypeDescription -> Nullable<Varchar>,
        BrandId -> Nullable<Uuid>,
        #[max_length = 512]
        ColorDescription -> Nullable<Varchar>,
        LostInAirportId -> Nullable<Uuid>,
        LostReasonId -> Nullable<Uuid>,
    }
}

diesel::table! {
    BaggagePropertiesBasicElement (BaggagePropertyId, BaggageTypeId) {
        BaggagePropertyId -> Uuid,
        BaggageTypeId -> Uuid,
    }
}

diesel::table! {
    BaggagePropertiesExternalElement (BaggagePropertyId, BaggageTypeId) {
        BaggagePropertyId -> Uuid,
        BaggageTypeId -> Uuid,
    }
}

diesel::table! {
    BaggageType3DModelSidePictures (BaggageTypeId, Side) {
        BaggageTypeId -> Uuid,
        Side -> Text,
        #[max_length = 128]
        MimeType -> Nullable<Varchar>,
        Content -> Nullable<Bytea>,
    }
}

diesel::table! {
    BaggageType3DModels (ID) {
        ID -> Uuid,
        #[max_length = 10485760]
        ModelContent -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    BaggageTypeImages (BaggageTypeId, IsThumbnail, IsColored) {
        BaggageTypeId -> Uuid,
        IsThumbnail -> Bool,
        IsColored -> Bool,
        #[max_length = 64]
        MimeType -> Varchar,
        Content -> Bytea,
    }
}

diesel::table! {
    BaggageTypeTranslation (ID) {
        ID -> Uuid,
        BaggageTypeId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    BaggageTypes (ID) {
        ID -> Uuid,
        TypeGroup -> Nullable<Int2>,
        #[max_length = 5]
        Code -> Varchar,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    Brands (ID) {
        ID -> Uuid,
        #[max_length = 256]
        Name -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    BsmRushDeliveries (ID) {
        ID -> Uuid,
        #[max_length = 64]
        BsmRushId -> Varchar,
        #[max_length = 64]
        BagTag -> Varchar,
        PlaceQTY -> Int4,
        Weight -> Numeric,
        #[max_length = 128]
        Surname -> Nullable<Varchar>,
        #[max_length = 128]
        Name -> Nullable<Varchar>,
        LostBaggagePlaceId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    BsmRushDeliveryFlights (ID) {
        ID -> Uuid,
        BsmRushDeliveryId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    CalendarDates (ID) {
        ID -> Int4,
        Date -> Date,
        Year -> Int2,
        Month -> Int2,
        DayOfMonth -> Int2,
        DayOfYear -> Int2,
        Quarter -> Int2,
        Week -> Int2,
        WeekDay -> Int2,
    }
}

diesel::table! {
    CarrierTranslation (ID) {
        ID -> Uuid,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    Carriers (ID) {
        ID -> Uuid,
        #[max_length = 5]
        IATA -> Nullable<Varchar>,
        #[max_length = 5]
        ICAO -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    CaseBaggageFlights (ID) {
        ID -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        CaseId -> Uuid,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    CaseMatches (LostBaggageCaseId, UnclaimedBaggageCaseId) {
        LostBaggageCaseId -> Uuid,
        UnclaimedBaggageCaseId -> Uuid,
        MatchingScore -> Float8,
        TagMatches -> Bool,
    }
}

diesel::table! {
    CaseMatchingCoefficients (Id) {
        FirstName -> Float8,
        LastName -> Float8,
        Initials -> Float8,
        Phone -> Float8,
        PermanentAddress -> Float8,
        TemporaryAddress -> Float8,
        Email -> Float8,
        FlightNumber -> Float8,
        Carrier -> Float8,
        DepartureAirport -> Float8,
        ArrivalAirport -> Float8,
        BaggageTagNumber -> Float8,
        BaggageType -> Float8,
        Color -> Float8,
        Material -> Float8,
        BasicElements -> Float8,
        ExternalElements -> Float8,
        BaggageCategories -> Float8,
        BaggageWeight -> Float8,
        Brand -> Float8,
        Id -> Int4,
    }
}

diesel::table! {
    CaseNumbers (OrganizationId, CarrierId) {
        OrganizationId -> Uuid,
        CarrierId -> Uuid,
        CurrentNumber -> Int4,
    }
}

diesel::table! {
    CasePassengerFlights (ID) {
        ID -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        CaseId -> Uuid,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    CaseRequesters (Id) {
        Id -> Uuid,
        #[max_length = 256]
        Surname -> Nullable<Varchar>,
        #[max_length = 128]
        Name -> Nullable<Varchar>,
        #[max_length = 128]
        SecondName -> Nullable<Varchar>,
        #[max_length = 32]
        Initials -> Nullable<Varchar>,
        #[max_length = 1]
        ServiceClass -> Nullable<Varchar>,
        #[max_length = 16]
        Language -> Nullable<Varchar>,
        #[max_length = 32]
        PhoneNumber -> Nullable<Varchar>,
        #[max_length = 32]
        BookingNumber -> Nullable<Varchar>,
        #[max_length = 32]
        TicketNumber -> Nullable<Varchar>,
        #[max_length = 256]
        PermanentAddress -> Nullable<Varchar>,
        #[max_length = 256]
        TemporaryAddress -> Nullable<Varchar>,
        #[max_length = 256]
        EmailAddress -> Nullable<Varchar>,
        #[max_length = 32]
        CarrierLoyaltyCard -> Nullable<Varchar>,
        #[max_length = 32]
        AdditionalPhoneNumber -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        CaseId -> Uuid,
        Gender -> Nullable<Int2>,
        TemporaryDateEnd -> Nullable<Timestamptz>,
        TemporaryDateStart -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    CaseStatus (Id) {
        Id -> Int2,
        #[max_length = 32]
        Name -> Varchar,
        #[max_length = 32]
        NameEN -> Varchar,
    }
}

diesel::table! {
    Cases (ID) {
        ID -> Uuid,
        CaseStatus -> Nullable<Int2>,
        BaggageCaseType -> Int2,
        #[max_length = 16]
        CaseNumber -> Nullable<Varchar>,
        SuspendedDate -> Nullable<Timestamptz>,
        ArchiveDate -> Nullable<Timestamptz>,
        RenewalDate -> Nullable<Timestamptz>,
        ClosedDate -> Nullable<Timestamptz>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        OrganizationId -> Nullable<Uuid>,
        #[max_length = 1500]
        LostReasonCommentary -> Nullable<Varchar>,
        LostReasonId -> Nullable<Uuid>,
        #[max_length = 32]
        TotalWeight -> Nullable<Varchar>,
        LostInAirportId -> Nullable<Uuid>,
        CaseAirportId -> Nullable<Uuid>,
        CaseCarrierId -> Nullable<Uuid>,
        TotalReceivedPlaces -> Nullable<Int4>,
        TotalReceivedWeight -> Nullable<Numeric>,
        TotalRegisteredPlaces -> Nullable<Int4>,
        TotalRegisteredWeight -> Nullable<Numeric>,
        HasHighImportanceMark -> Bool,
        #[max_length = 50]
        HighImportanceMarkComment -> Nullable<Varchar>,
        ExternalSourceDocumentId -> Nullable<Uuid>,
    }
}

diesel::table! {
    CasesHistory (Id) {
        Id -> Uuid,
        Status -> Nullable<Int2>,
        CaseId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        #[max_length = 3000]
        Message -> Nullable<Varchar>,
    }
}

diesel::table! {
    Comment (ID) {
        ID -> Uuid,
        CommentText -> Nullable<Text>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    CommercialActHistory (ID) {
        ID -> Uuid,
        AbstractCommercialActId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        #[max_length = 3000]
        Message -> Nullable<Varchar>,
    }
}

diesel::table! {
    CommercialActItemDamageDetails (ID) {
        ID -> Uuid,
        ItemId -> Uuid,
        DamageNumber -> Int4,
        DamageReasonId -> Uuid,
        DamagedSide -> Nullable<Text>,
        #[max_length = 10]
        DamageType -> Nullable<Varchar>,
        #[max_length = 10]
        DamageDegree -> Nullable<Varchar>,
        #[max_length = 4000]
        Description -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    CommercialActItems (ID) {
        ID -> Uuid,
        CommercialActId -> Uuid,
        ItemNumber -> Int2,
        SourcePlaceId -> Uuid,
        #[max_length = 16]
        BaggageTagNumber -> Nullable<Varchar>,
        #[max_length = 16]
        DeliveryTagNumber -> Nullable<Varchar>,
        #[max_length = 32]
        BaggageType -> Nullable<Varchar>,
        #[max_length = 512]
        PackingCondition -> Nullable<Varchar>,
        Introscoped -> Bool,
        BaggageOpened -> Bool,
        HasDamages -> Bool,
        Cost -> Numeric,
        ContentCost -> Numeric,
        IsSealed -> Bool,
        #[max_length = 200]
        SealNumber -> Nullable<Varchar>,
        #[max_length = 1024]
        AdditionalInformation -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    CommercialActPassengerFlights (ID) {
        ID -> Uuid,
        CommercialActId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    CommercialActPassengers (ID) {
        ID -> Uuid,
        CommercialActId -> Uuid,
        #[max_length = 256]
        Surname -> Varchar,
        #[max_length = 32]
        TicketNumber -> Nullable<Varchar>,
        #[max_length = 32]
        BookingNumber -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    CommercialActs (ID) {
        ID -> Uuid,
        Type -> Int2,
        #[max_length = 64]
        ActNumber -> Varchar,
        #[max_length = 16]
        DocumentNumber -> Nullable<Varchar>,
        OrganizationId -> Uuid,
        AirportId -> Uuid,
        CarrierId -> Uuid,
        Reason -> Nullable<Int2>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        SourceDocumentId -> Uuid,
        CurrencyId -> Nullable<Uuid>,
    }
}

diesel::table! {
    Currency (ID) {
        ID -> Uuid,
        #[max_length = 3]
        ISO3Code -> Varchar,
        #[max_length = 3]
        Sign -> Varchar,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    CurrencyTranslation (ID) {
        ID -> Uuid,
        CurrencyId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    DamageReasonTranslation (ID) {
        ID -> Uuid,
        DamageReasonId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    DamageReasons (ID) {
        ID -> Uuid,
        #[max_length = 10]
        Code -> Varchar,
        ParentId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggage (ID) {
        ID -> Uuid,
        OrganizationId -> Uuid,
        AirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        Status -> Int2,
        #[max_length = 16]
        DamagedBaggageNumber -> Nullable<Varchar>,
        ArchiveDate -> Nullable<Timestamptz>,
        GuiltyAirportId -> Nullable<Uuid>,
        CurrencyId -> Uuid,
        TotalAmount -> Numeric,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        #[max_length = 32]
        TotalWeight -> Nullable<Varchar>,
        HasHighImportanceMark -> Bool,
        #[max_length = 50]
        HighImportanceMarkComment -> Nullable<Varchar>,
    }
}

diesel::table! {
    DamagedBaggageContents (ID) {
        ID -> Uuid,
        PlaceId -> Uuid,
        BaggageCategoryId -> Nullable<Int4>,
        #[max_length = 64]
        BaggageObject -> Nullable<Varchar>,
        #[max_length = 512]
        BaggageObjectDescription -> Nullable<Varchar>,
        Cost -> Nullable<Numeric>,
        #[max_length = 10]
        DamageDegree -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggageDamageInfo (ID) {
        ID -> Uuid,
        DamagedBaggagePlaceId -> Uuid,
        AssignedToModelSide -> Nullable<Text>,
        AssignedToPhotoId -> Nullable<Uuid>,
        Coord_X -> Nullable<Float8>,
        Coord_Y -> Nullable<Float8>,
        #[max_length = 10]
        DamageType -> Nullable<Varchar>,
        DamageReasonId -> Nullable<Uuid>,
        #[max_length = 10]
        DamageDegree -> Nullable<Varchar>,
        #[max_length = 4000]
        Description -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Coord_Z -> Nullable<Float8>,
        DamageNumber -> Int4,
    }
}

diesel::table! {
    DamagedBaggageDamageInfoPhotos (ID) {
        ID -> Uuid,
        DamagedBaggageDamageInfoId -> Uuid,
        #[max_length = 128]
        MimeType -> Nullable<Varchar>,
        Size -> Int4,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggageFlights (ID) {
        ID -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
        DamagedBaggageId -> Uuid,
    }
}

diesel::table! {
    DamagedBaggageHistory (ID) {
        ID -> Uuid,
        DamagedBaggageId -> Uuid,
        Status -> Nullable<Int2>,
        #[max_length = 1000]
        Message -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggageNumbers (OrganizationId, CarrierId) {
        OrganizationId -> Uuid,
        CarrierId -> Uuid,
        CurrentNumber -> Int4,
    }
}

diesel::table! {
    DamagedBaggageOwners (ID) {
        ID -> Uuid,
        PlaceId -> Uuid,
        #[max_length = 128]
        Surname -> Nullable<Varchar>,
        #[max_length = 64]
        Name -> Nullable<Varchar>,
        #[max_length = 64]
        SecondName -> Nullable<Varchar>,
        #[max_length = 16]
        Initials -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggagePassengerFlights (ID) {
        ID -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
        DamagedBaggageId -> Uuid,
    }
}

diesel::table! {
    DamagedBaggagePlacePhotos (ID) {
        ID -> Uuid,
        DamagedBaggagePlaceId -> Nullable<Uuid>,
        #[max_length = 128]
        MimeType -> Nullable<Varchar>,
        Size -> Int4,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggagePlaces (ID) {
        ID -> Uuid,
        DamagedBaggageId -> Uuid,
        PlaceNumber -> Int2,
        DamagedInAirportId -> Nullable<Uuid>,
        WearDegree -> Nullable<Int2>,
        PurchaseYear -> Nullable<Int4>,
        BoxCost -> Nullable<Numeric>,
        TotalCost -> Nullable<Numeric>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggageProperties (ID) {
        ID -> Uuid,
        PlaceId -> Uuid,
        ColorId -> Uuid,
        #[max_length = 500]
        ColorDescription -> Nullable<Varchar>,
        BaggageTypeId -> Uuid,
        #[max_length = 500]
        BaggageTypeDescription -> Nullable<Varchar>,
        MaterialId -> Uuid,
        #[max_length = 16]
        BaggageTagNumber -> Nullable<Varchar>,
        #[max_length = 16]
        DeliveryTagNumber -> Nullable<Varchar>,
        BaggageWeight -> Nullable<Numeric>,
        BrandId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggagePropertiesBasicElement (DamagedBaggagePropertyId, BaggageTypeId) {
        DamagedBaggagePropertyId -> Uuid,
        BaggageTypeId -> Uuid,
    }
}

diesel::table! {
    DamagedBaggagePropertiesExternalElement (DamagedBaggagePropertyId, BaggageTypeId) {
        DamagedBaggagePropertyId -> Uuid,
        BaggageTypeId -> Uuid,
    }
}

diesel::table! {
    DamagedBaggageRequesters (ID) {
        ID -> Uuid,
        DamagedBaggageId -> Uuid,
        #[max_length = 256]
        Surname -> Nullable<Varchar>,
        #[max_length = 128]
        Name -> Nullable<Varchar>,
        #[max_length = 128]
        SecondName -> Nullable<Varchar>,
        #[max_length = 32]
        Initials -> Nullable<Varchar>,
        Gender -> Nullable<Int2>,
        #[max_length = 1]
        ServiceClass -> Nullable<Varchar>,
        #[max_length = 16]
        Language -> Nullable<Varchar>,
        #[max_length = 32]
        PhoneNumber -> Nullable<Varchar>,
        #[max_length = 32]
        BookingNumber -> Nullable<Varchar>,
        #[max_length = 32]
        TicketNumber -> Nullable<Varchar>,
        #[max_length = 256]
        PermanentAddress -> Nullable<Varchar>,
        #[max_length = 256]
        TemporaryAddress -> Nullable<Varchar>,
        TemporaryDateStart -> Nullable<Timestamptz>,
        TemporaryDateEnd -> Nullable<Timestamptz>,
        #[max_length = 128]
        EmailAddress -> Nullable<Varchar>,
        #[max_length = 32]
        CarrierLoyaltyCard -> Nullable<Varchar>,
        #[max_length = 32]
        AdditionalPhoneNumber -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedBaggageSettings (ID) {
        ID -> Uuid,
        DamagedBaggageId -> Uuid,
        SendNotificationToPassengerByEmail -> Bool,
        SendNotificationToPassengerBySms -> Bool,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DamagedCommercialActCheckResults (CommercialActId) {
        CommercialActId -> Uuid,
        PlacesRegistered -> Int2,
        PlacesDamaged -> Int2,
        RegisteredWeight -> Numeric,
        DamagedWeight -> Numeric,
    }
}

diesel::table! {
    DataProtectionKeys (Id) {
        Id -> Int4,
        FriendlyName -> Nullable<Text>,
        Xml -> Nullable<Text>,
    }
}

diesel::table! {
    DelayCodeTranslation (ID) {
        ID -> Uuid,
        DelayCodeId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    DelayCodes (ID) {
        ID -> Uuid,
        ParentId -> Nullable<Uuid>,
        #[max_length = 10]
        Code -> Varchar,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DocumentAttachment (Id) {
        Id -> Uuid,
        DocumentId -> Uuid,
        Type -> Int2,
        ActualVersionId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Uuid,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DocumentAttachmentVersion (Id) {
        Id -> Uuid,
        AttachmentId -> Uuid,
        #[max_length = 256]
        Name -> Varchar,
        #[max_length = 128]
        MimeType -> Varchar,
        Size -> Int4,
        CreatedAt -> Timestamptz,
        CreatedBy -> Uuid,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    DocumentsLinks (SourceDocumentId, TargetDocumentId, SourcePlaceId, TargetPlaceId) {
        SourceDocumentId -> Uuid,
        SourcePlaceId -> Uuid,
        TargetDocumentId -> Uuid,
        TargetPlaceId -> Uuid,
        SourceDocumentType -> Int2,
        TargetDocumentType -> Int2,
        LinkType -> Int2,
    }
}

diesel::table! {
    ExternalSystemSourceDocuments (ID) {
        ID -> Uuid,
        #[max_length = 10]
        ExternalSourceSystem -> Nullable<Varchar>,
        #[max_length = 128]
        SourceNumber -> Nullable<Varchar>,
        TargetDocumentType -> Nullable<Int4>,
        TargetDocumentId -> Uuid,
        SourceData -> Nullable<Text>,
        SourceSpecificInfo -> Nullable<Text>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    MessageNotificationChannel (MessageId, Channel) {
        Channel -> Int2,
        MessageId -> Uuid,
    }
}

diesel::table! {
    Messages (ID) {
        ID -> Uuid,
        CaseId -> Nullable<Uuid>,
        #[max_length = 64]
        CaseNumber -> Nullable<Varchar>,
        CaseAirportId -> Nullable<Uuid>,
        CaseCarrierId -> Nullable<Uuid>,
        ParentMessageId -> Nullable<Uuid>,
        IsRead -> Bool,
        #[max_length = 4000]
        MessageText -> Nullable<Varchar>,
        MessageType -> Int2,
        #[max_length = 256]
        SenderFullName -> Nullable<Varchar>,
        RequestorId -> Nullable<Uuid>,
        ForwardedToAirportId -> Nullable<Uuid>,
        ForwardedByCarrierId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        OrganizationId -> Nullable<Uuid>,
        RecepientId -> Nullable<Uuid>,
        RecepientOrganizationId -> Nullable<Uuid>,
        IsReadByRecipient -> Bool,
        BaggageId -> Nullable<Uuid>,
        DeliveryTagNumber -> Nullable<Text>,
        LostBaggageCaseId -> Nullable<Uuid>,
        LostBaggagePlaceId -> Nullable<Uuid>,
        BsmRushDeliveryId -> Nullable<Uuid>,
        IsHidden -> Bool,
        IsHiddenByRecipient -> Bool,
    }
}

diesel::table! {
    OrganizationAdditionalContacts (Id) {
        Id -> Uuid,
        OrganizationId -> Nullable<Uuid>,
        #[max_length = 2048]
        Address -> Nullable<Varchar>,
        ContactAddressType -> Nullable<Int4>,
        #[max_length = 2048]
        Link -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    OrganizationNotification (Id) {
        Id -> Uuid,
        OrganizationId -> Uuid,
        TypeId -> Int4,
    }
}

diesel::table! {
    OrganizationNotificationChannel (Id) {
        Id -> Uuid,
        OrganizationNotificationId -> Uuid,
        ChannelId -> Int4,
        IsEnabled -> Bool,
    }
}

diesel::table! {
    OrganizationUser (Id) {
        Id -> Uuid,
        OrganizationId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    OrganizationUsersAirports (OrganizationUserId, AirportId) {
        OrganizationUserId -> Uuid,
        AirportId -> Uuid,
    }
}

diesel::table! {
    OrganizationUsersCarriers (OrganizationUserId, CarrierId) {
        OrganizationUserId -> Uuid,
        CarrierId -> Uuid,
    }
}

diesel::table! {
    Organizations (ID) {
        ID -> Uuid,
        OrganizationType -> Int2,
        #[max_length = 32]
        INN -> Nullable<Varchar>,
        #[max_length = 32]
        KPP -> Nullable<Varchar>,
        #[max_length = 32]
        OGRN -> Nullable<Varchar>,
        #[max_length = 256]
        LegalAddress -> Nullable<Varchar>,
        #[max_length = 256]
        PostalAddress -> Nullable<Varchar>,
        #[max_length = 256]
        ContactPersonFIO -> Nullable<Varchar>,
        #[max_length = 32]
        ContactPhone -> Nullable<Varchar>,
        #[max_length = 128]
        ContactEmail -> Nullable<Varchar>,
        #[max_length = 256]
        BankName -> Nullable<Varchar>,
        #[max_length = 128]
        CheckingAccount -> Nullable<Varchar>,
        #[max_length = 128]
        CorrespondentAccount -> Nullable<Varchar>,
        #[max_length = 32]
        BIK -> Nullable<Varchar>,
        CaseCreated -> Nullable<Bool>,
        CaseNonClaimedBaggageCreated -> Nullable<Bool>,
        FiveDaysAfterCreation -> Nullable<Bool>,
        CaseClosedAfterFiveDays -> Nullable<Bool>,
        SendByEmail -> Nullable<Bool>,
        SendInternally -> Nullable<Bool>,
        AirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        #[max_length = 256]
        Name -> Varchar,
        SMSCount -> Int4,
        PrintAirportContactsInDocs -> Nullable<Bool>,
        DeliveryBaggageCreation -> Nullable<Bool>,
        NotifyPassenger_ByEmail -> Nullable<Bool>,
        NotifyPassenger_BySMS -> Nullable<Bool>,
        NotifyPassenger_OverrideDocumentSettings -> Nullable<Bool>,
        DamagedBaggageCreated -> Nullable<Bool>,
        ComplexMatching -> Bool,
        CanSendDelivery -> Nullable<Bool>,
        AllowUseXHInCarriers -> Bool,
    }
}

diesel::table! {
    OrganizationsAirports (OrganizationId, AirportId) {
        OrganizationId -> Uuid,
        AirportId -> Uuid,
    }
}

diesel::table! {
    OrganizationsCarriers (OrganizationId, CarrierId) {
        OrganizationId -> Uuid,
        CarrierId -> Uuid,
    }
}

diesel::table! {
    PhotoPacketItems (ID) {
        ID -> Uuid,
        PacketId -> Uuid,
        #[max_length = 128]
        MimeType -> Nullable<Varchar>,
        Size -> Int4,
        Thumbnail -> Bytea,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    PhotoPackets (ID) {
        ID -> Uuid,
        OrganizationId -> Uuid,
        PacketType -> Int2,
        #[max_length = 256]
        Name -> Nullable<Varchar>,
        ExpiresAt -> Timestamptz,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        EditStartTime -> Nullable<Timestamptz>,
        IsInEdit -> Bool,
    }
}

diesel::table! {
    PrintFormHeaderFooter (ID) {
        ID -> Uuid,
        #[max_length = 100]
        HeaderLine1 -> Nullable<Varchar>,
        #[max_length = 100]
        HeaderLine2 -> Nullable<Varchar>,
        #[max_length = 100]
        HeaderLine3 -> Nullable<Varchar>,
        #[max_length = 150]
        Footer -> Nullable<Varchar>,
    }
}

diesel::table! {
    QuickDeliveries (ID) {
        ID -> Uuid,
        OrganizationId -> Uuid,
        #[max_length = 32]
        QuickDeliveryNumber -> Varchar,
        Status -> Nullable<Int2>,
        AirportId -> Uuid,
        CarrierId -> Uuid,
        SourceCarrierId -> Uuid,
        LostReasonId -> Nullable<Uuid>,
        LostInAirportId -> Nullable<Uuid>,
        DeliveryStartedAt -> Nullable<Timestamptz>,
        DeliveryStartedBy -> Nullable<Uuid>,
        DeliveredAt -> Nullable<Timestamptz>,
        DeliveredBy -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        IssuedAt -> Nullable<Timestamptz>,
        IssuedBy -> Nullable<Uuid>,
        ArchiveDate -> Nullable<Timestamptz>,
        ExternalSourceDocumentId -> Nullable<Uuid>,
    }
}

diesel::table! {
    QuickDeliveryBaggagePlaces (ID) {
        ID -> Uuid,
        QuickDeliveryId -> Uuid,
        PlaceNumber -> Int4,
        #[max_length = 32]
        BaggageTagNumber -> Varchar,
        #[max_length = 32]
        DeliveryTagNumber -> Varchar,
        #[max_length = 512]
        FirstName -> Nullable<Varchar>,
        #[max_length = 32]
        OriginalFlightNumber -> Varchar,
        IsDelivered -> Bool,
        DeliveryChangedAt -> Nullable<Timestamptz>,
        DeliverySetBy -> Nullable<Uuid>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        LostBaggageCaseId -> Nullable<Uuid>,
        LostBaggagePlaceId -> Nullable<Uuid>,
        DeliveryCost -> Numeric,
        DeliveryCurrencyId -> Nullable<Uuid>,
        IssueType -> Nullable<Int2>,
        MatchingPoints -> Float8,
    }
}

diesel::table! {
    QuickDeliveryFlights (ID) {
        ID -> Uuid,
        QuickDeliveryId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
        NotifyCarrier -> Bool,
    }
}

diesel::table! {
    QuickDeliveryHistory (Id) {
        Id -> Uuid,
        QuickDeliveryId -> Uuid,
        Status -> Int2,
        Message -> Nullable<Text>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    SupportRequestNumber (Id) {
        CurrentNumber -> Int4,
        Id -> Int4,
    }
}

diesel::table! {
    ThingCategories (ID) {
        ID -> Uuid,
        BaggageTypeId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ThingCategoryTranslation (ID) {
        ID -> Uuid,
        CategoryId -> Nullable<Uuid>,
        #[max_length = 2]
        Lang -> Nullable<Varchar>,
        #[max_length = 512]
        Value -> Nullable<Varchar>,
    }
}

diesel::table! {
    ThingFlights (ID) {
        ID -> Uuid,
        ThingId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    ThingPassengers (ID) {
        ID -> Uuid,
        ThingId -> Uuid,
        #[max_length = 256]
        Surname -> Nullable<Varchar>,
        #[max_length = 128]
        Name -> Nullable<Varchar>,
        #[max_length = 128]
        SecondName -> Nullable<Varchar>,
        #[max_length = 32]
        Initials -> Nullable<Varchar>,
        Gender -> Nullable<Int2>,
        #[max_length = 1]
        ServiceClass -> Nullable<Varchar>,
        #[max_length = 16]
        Language -> Nullable<Varchar>,
        #[max_length = 32]
        PhoneNumber -> Nullable<Varchar>,
        #[max_length = 32]
        BookingNumber -> Nullable<Varchar>,
        #[max_length = 32]
        TicketNumber -> Nullable<Varchar>,
        #[max_length = 32]
        PassengerPlaceNumber -> Nullable<Varchar>,
        #[max_length = 256]
        EmailAddress -> Nullable<Varchar>,
        #[max_length = 32]
        AdditionalPhoneNumber -> Nullable<Varchar>,
        NotifyByEmail -> Bool,
        NotifyBySMS -> Bool,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ThingPhotos (Id) {
        Id -> Uuid,
        ThingPropertiesId -> Uuid,
        #[max_length = 128]
        MimeType -> Nullable<Varchar>,
        Size -> Int4,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ThingProperties (ID) {
        ID -> Uuid,
        ThingId -> Uuid,
        PropertyType -> Nullable<Int2>,
        ColorId -> Uuid,
        BaggageTypeId -> Uuid,
        MaterialId -> Uuid,
        CategoryId -> Nullable<Uuid>,
        BrandId -> Nullable<Uuid>,
        #[max_length = 512]
        ColorDescription -> Nullable<Varchar>,
        #[max_length = 512]
        Description -> Nullable<Varchar>,
        #[max_length = 512]
        StoragePlace -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ThingPropertiesBasicElement (ThingPropertiesId, BaggageTypeId) {
        ThingPropertiesId -> Uuid,
        BaggageTypeId -> Uuid,
    }
}

diesel::table! {
    ThingPropertiesExternalElement (ThingPropertiesId, BaggageTypeId) {
        ThingPropertiesId -> Uuid,
        BaggageTypeId -> Uuid,
    }
}

diesel::table! {
    Things (ID) {
        ID -> Uuid,
        Status -> Nullable<Int2>,
        ThingType -> Int2,
        OrganizationId -> Uuid,
        #[max_length = 16]
        DocumentNumber -> Nullable<Varchar>,
        SuspendedDate -> Nullable<Timestamptz>,
        ArchiveDate -> Nullable<Timestamptz>,
        RenewalDate -> Nullable<Timestamptz>,
        ClosedDate -> Nullable<Timestamptz>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        AirportId -> Uuid,
        CarrierId -> Uuid,
        ClosedBy -> Nullable<Uuid>,
        DeliveredAt -> Nullable<Timestamptz>,
        DeliveredBy -> Nullable<Uuid>,
        DeliveryStartedAt -> Nullable<Timestamptz>,
        DeliveryStartedBy -> Nullable<Uuid>,
        SuspendedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ThingsHistory (Id) {
        Id -> Uuid,
        ThingId -> Uuid,
        Status -> Nullable<Int2>,
        #[max_length = 3000]
        Message -> Nullable<Varchar>,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
    }
}

diesel::table! {
    ThingsMatches (LostThingId, FoundThingId) {
        LostThingId -> Uuid,
        FoundThingId -> Uuid,
        MatchingScore -> Float8,
    }
}

diesel::table! {
    ThingsMatchingCoefficients (Id) {
        FirstName -> Float8,
        LastName -> Float8,
        Initials -> Float8,
        Phone -> Float8,
        PermanentAddress -> Float8,
        TemporaryAddress -> Float8,
        Email -> Float8,
        FlightNumber -> Float8,
        Carrier -> Float8,
        DepartureAirport -> Float8,
        ArrivalAirport -> Float8,
        BaggageType -> Float8,
        Color -> Float8,
        Brand -> Float8,
        Id -> Int4,
    }
}

diesel::table! {
    UnclaimedBaggageDeliveryFlights (ID) {
        ID -> Uuid,
        CaseId -> Uuid,
        LostBaggagePlaceId -> Uuid,
        CreatedAt -> Timestamptz,
        CreatedBy -> Nullable<Uuid>,
        UpdatedAt -> Nullable<Timestamptz>,
        UpdatedBy -> Nullable<Uuid>,
        IsDeleted -> Bool,
        DeletedAt -> Nullable<Timestamptz>,
        DeletedBy -> Nullable<Uuid>,
        Position -> Int2,
        DepartureAirportId -> Nullable<Uuid>,
        ArrivalAirportId -> Nullable<Uuid>,
        CarrierId -> Nullable<Uuid>,
        #[max_length = 16]
        FlightNumber -> Nullable<Varchar>,
        DepartureDate -> Nullable<Timestamptz>,
        ArrivalDate -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    UnclaimedCommercialActCheckResults (CommercialActId) {
        CommercialActId -> Uuid,
        PlacesRegistered -> Int2,
        PlacesReceived -> Int2,
        PlacesLost -> Int2,
        RegisteredWeight -> Numeric,
        ReceivedWeight -> Numeric,
        LostWeight -> Numeric,
    }
}

diesel::table! {
    UserActionsHistory (Id) {
        Id -> Uuid,
        UserId -> Uuid,
        ActionType -> Nullable<Int2>,
        #[max_length = 256]
        Message -> Nullable<Varchar>,
        Content -> Nullable<Text>,
        CreatedAt -> Timestamptz,
        DocumentId -> Nullable<Uuid>,
    }
}

diesel::joinable!(AdditionalContactsSchedule -> OrganizationAdditionalContacts (AdditionalContactId));
diesel::joinable!(AdditionalSettings -> Airports (ArrivalAirportId));
diesel::joinable!(AdditionalSettings -> Cases (CaseId));
diesel::joinable!(AirportTranslation -> Airports (AirportId));
diesel::joinable!(BaggageContents -> BaggageContentCategories (BaggageCategoryId));
diesel::joinable!(BaggageContents -> BaggagePlaces (BaggagePlaceId));
diesel::joinable!(BaggageOwners -> BaggagePlaces (BaggagePlaceId));
diesel::joinable!(BaggagePhotos -> BaggagePlaces (BaggagePlaceId));
diesel::joinable!(BaggagePlaces -> Airports (LostInAirportId));
diesel::joinable!(BaggagePlaces -> Cases (CaseId));
diesel::joinable!(BaggagePlaces -> DelayCodes (LostReasonId));
diesel::joinable!(BaggageProperties -> BaggagePlaces (BaggagePlaceId));
diesel::joinable!(BaggageProperties -> Brands (BrandId));
diesel::joinable!(BaggagePropertiesBasicElement -> BaggageProperties (BaggagePropertyId));
diesel::joinable!(BaggagePropertiesBasicElement -> BaggageTypes (BaggageTypeId));
diesel::joinable!(BaggagePropertiesExternalElement -> BaggageProperties (BaggagePropertyId));
diesel::joinable!(BaggagePropertiesExternalElement -> BaggageTypes (BaggageTypeId));
diesel::joinable!(BaggageType3DModelSidePictures -> BaggageTypes (BaggageTypeId));
diesel::joinable!(BaggageType3DModels -> BaggageTypes (ID));
diesel::joinable!(BaggageTypeImages -> BaggageTypes (BaggageTypeId));
diesel::joinable!(BaggageTypeTranslation -> BaggageTypes (BaggageTypeId));
diesel::joinable!(BsmRushDeliveries -> BaggagePlaces (LostBaggagePlaceId));
diesel::joinable!(BsmRushDeliveryFlights -> BsmRushDeliveries (BsmRushDeliveryId));
diesel::joinable!(BsmRushDeliveryFlights -> Carriers (CarrierId));
diesel::joinable!(CarrierTranslation -> Carriers (CarrierId));
diesel::joinable!(CaseBaggageFlights -> Carriers (CarrierId));
diesel::joinable!(CaseBaggageFlights -> Cases (CaseId));
diesel::joinable!(CasePassengerFlights -> Carriers (CarrierId));
diesel::joinable!(CasePassengerFlights -> Cases (CaseId));
diesel::joinable!(CaseRequesters -> Cases (CaseId));
diesel::joinable!(Cases -> Carriers (CaseCarrierId));
diesel::joinable!(Cases -> DelayCodes (LostReasonId));
diesel::joinable!(Cases -> ExternalSystemSourceDocuments (ExternalSourceDocumentId));
diesel::joinable!(Cases -> Organizations (OrganizationId));
diesel::joinable!(CasesHistory -> Cases (CaseId));
diesel::joinable!(CommercialActHistory -> CommercialActs (AbstractCommercialActId));
diesel::joinable!(CommercialActItemDamageDetails -> CommercialActItems (ItemId));
diesel::joinable!(CommercialActItemDamageDetails -> DamageReasons (DamageReasonId));
diesel::joinable!(CommercialActItems -> CommercialActs (CommercialActId));
diesel::joinable!(CommercialActPassengerFlights -> Carriers (CarrierId));
diesel::joinable!(CommercialActPassengerFlights -> CommercialActs (CommercialActId));
diesel::joinable!(CommercialActPassengers -> CommercialActs (CommercialActId));
diesel::joinable!(CommercialActs -> Airports (AirportId));
diesel::joinable!(CommercialActs -> Carriers (CarrierId));
diesel::joinable!(CommercialActs -> Currency (CurrencyId));
diesel::joinable!(CommercialActs -> Organizations (OrganizationId));
diesel::joinable!(CurrencyTranslation -> Currency (CurrencyId));
diesel::joinable!(DamageReasonTranslation -> DamageReasons (DamageReasonId));
diesel::joinable!(DamagedBaggage -> Carriers (CarrierId));
diesel::joinable!(DamagedBaggage -> Currency (CurrencyId));
diesel::joinable!(DamagedBaggage -> Organizations (OrganizationId));
diesel::joinable!(DamagedBaggageContents -> BaggageContentCategories (BaggageCategoryId));
diesel::joinable!(DamagedBaggageContents -> DamagedBaggagePlaces (PlaceId));
diesel::joinable!(DamagedBaggageDamageInfo -> DamageReasons (DamageReasonId));
diesel::joinable!(DamagedBaggageDamageInfo -> DamagedBaggagePlacePhotos (AssignedToPhotoId));
diesel::joinable!(DamagedBaggageDamageInfo -> DamagedBaggagePlaces (DamagedBaggagePlaceId));
diesel::joinable!(DamagedBaggageDamageInfoPhotos -> DamagedBaggageDamageInfo (DamagedBaggageDamageInfoId));
diesel::joinable!(DamagedBaggageFlights -> Carriers (CarrierId));
diesel::joinable!(DamagedBaggageFlights -> DamagedBaggage (DamagedBaggageId));
diesel::joinable!(DamagedBaggageHistory -> DamagedBaggage (DamagedBaggageId));
diesel::joinable!(DamagedBaggageOwners -> DamagedBaggagePlaces (PlaceId));
diesel::joinable!(DamagedBaggagePassengerFlights -> Carriers (CarrierId));
diesel::joinable!(DamagedBaggagePassengerFlights -> DamagedBaggage (DamagedBaggageId));
diesel::joinable!(DamagedBaggagePlacePhotos -> DamagedBaggagePlaces (DamagedBaggagePlaceId));
diesel::joinable!(DamagedBaggagePlaces -> Airports (DamagedInAirportId));
diesel::joinable!(DamagedBaggagePlaces -> DamagedBaggage (DamagedBaggageId));
diesel::joinable!(DamagedBaggageProperties -> Brands (BrandId));
diesel::joinable!(DamagedBaggageProperties -> DamagedBaggagePlaces (PlaceId));
diesel::joinable!(DamagedBaggagePropertiesBasicElement -> BaggageTypes (BaggageTypeId));
diesel::joinable!(DamagedBaggagePropertiesBasicElement -> DamagedBaggageProperties (DamagedBaggagePropertyId));
diesel::joinable!(DamagedBaggagePropertiesExternalElement -> BaggageTypes (BaggageTypeId));
diesel::joinable!(DamagedBaggagePropertiesExternalElement -> DamagedBaggageProperties (DamagedBaggagePropertyId));
diesel::joinable!(DamagedBaggageRequesters -> DamagedBaggage (DamagedBaggageId));
diesel::joinable!(DamagedBaggageSettings -> DamagedBaggage (DamagedBaggageId));
diesel::joinable!(DamagedCommercialActCheckResults -> CommercialActs (CommercialActId));
diesel::joinable!(DelayCodeTranslation -> DelayCodes (DelayCodeId));
diesel::joinable!(DocumentAttachmentVersion -> DocumentAttachment (AttachmentId));
diesel::joinable!(MessageNotificationChannel -> Messages (MessageId));
diesel::joinable!(OrganizationAdditionalContacts -> Organizations (OrganizationId));
diesel::joinable!(OrganizationNotification -> Organizations (OrganizationId));
diesel::joinable!(OrganizationNotificationChannel -> OrganizationNotification (OrganizationNotificationId));
diesel::joinable!(OrganizationUser -> Organizations (OrganizationId));
diesel::joinable!(OrganizationUsersAirports -> Airports (AirportId));
diesel::joinable!(OrganizationUsersAirports -> OrganizationUser (OrganizationUserId));
diesel::joinable!(OrganizationUsersCarriers -> Carriers (CarrierId));
diesel::joinable!(OrganizationUsersCarriers -> OrganizationUser (OrganizationUserId));
diesel::joinable!(Organizations -> Airports (AirportId));
diesel::joinable!(Organizations -> Carriers (CarrierId));
diesel::joinable!(OrganizationsAirports -> Airports (AirportId));
diesel::joinable!(OrganizationsAirports -> Organizations (OrganizationId));
diesel::joinable!(OrganizationsCarriers -> Carriers (CarrierId));
diesel::joinable!(OrganizationsCarriers -> Organizations (OrganizationId));
diesel::joinable!(PhotoPacketItems -> PhotoPackets (PacketId));
diesel::joinable!(PrintFormHeaderFooter -> Organizations (ID));
diesel::joinable!(QuickDeliveries -> DelayCodes (LostReasonId));
diesel::joinable!(QuickDeliveries -> ExternalSystemSourceDocuments (ExternalSourceDocumentId));
diesel::joinable!(QuickDeliveries -> Organizations (OrganizationId));
diesel::joinable!(QuickDeliveryBaggagePlaces -> BaggagePlaces (LostBaggagePlaceId));
diesel::joinable!(QuickDeliveryBaggagePlaces -> Cases (LostBaggageCaseId));
diesel::joinable!(QuickDeliveryBaggagePlaces -> Currency (DeliveryCurrencyId));
diesel::joinable!(QuickDeliveryBaggagePlaces -> QuickDeliveries (QuickDeliveryId));
diesel::joinable!(QuickDeliveryFlights -> Carriers (CarrierId));
diesel::joinable!(QuickDeliveryFlights -> QuickDeliveries (QuickDeliveryId));
diesel::joinable!(QuickDeliveryHistory -> QuickDeliveries (QuickDeliveryId));
diesel::joinable!(ThingCategories -> BaggageTypes (BaggageTypeId));
diesel::joinable!(ThingCategoryTranslation -> ThingCategories (CategoryId));
diesel::joinable!(ThingFlights -> Carriers (CarrierId));
diesel::joinable!(ThingFlights -> Things (ThingId));
diesel::joinable!(ThingPassengers -> Things (ThingId));
diesel::joinable!(ThingPhotos -> ThingProperties (ThingPropertiesId));
diesel::joinable!(ThingProperties -> Brands (BrandId));
diesel::joinable!(ThingProperties -> ThingCategories (CategoryId));
diesel::joinable!(ThingProperties -> Things (ThingId));
diesel::joinable!(ThingPropertiesBasicElement -> BaggageTypes (BaggageTypeId));
diesel::joinable!(ThingPropertiesBasicElement -> ThingProperties (ThingPropertiesId));
diesel::joinable!(ThingPropertiesExternalElement -> BaggageTypes (BaggageTypeId));
diesel::joinable!(ThingPropertiesExternalElement -> ThingProperties (ThingPropertiesId));
diesel::joinable!(Things -> Airports (AirportId));
diesel::joinable!(Things -> Carriers (CarrierId));
diesel::joinable!(Things -> Organizations (OrganizationId));
diesel::joinable!(ThingsHistory -> Things (ThingId));
diesel::joinable!(UnclaimedBaggageDeliveryFlights -> BaggagePlaces (LostBaggagePlaceId));
diesel::joinable!(UnclaimedBaggageDeliveryFlights -> Carriers (CarrierId));
diesel::joinable!(UnclaimedBaggageDeliveryFlights -> Cases (CaseId));
diesel::joinable!(UnclaimedCommercialActCheckResults -> CommercialActs (CommercialActId));
diesel::joinable!(UserActionsHistory -> OrganizationUser (UserId));

diesel::allow_tables_to_appear_in_same_query!(
    AdditionalContactsSchedule,
    AdditionalSettings,
    Addresses,
    AirportTranslation,
    Airports,
    ArchivePolicySettings,
    BaggageContentCategories,
    BaggageContents,
    BaggageOwners,
    BaggagePhotos,
    BaggagePlaces,
    BaggageProperties,
    BaggagePropertiesBasicElement,
    BaggagePropertiesExternalElement,
    BaggageType3DModelSidePictures,
    BaggageType3DModels,
    BaggageTypeImages,
    BaggageTypeTranslation,
    BaggageTypes,
    Brands,
    BsmRushDeliveries,
    BsmRushDeliveryFlights,
    CalendarDates,
    CarrierTranslation,
    Carriers,
    CaseBaggageFlights,
    CaseMatches,
    CaseMatchingCoefficients,
    CaseNumbers,
    CasePassengerFlights,
    CaseRequesters,
    CaseStatus,
    Cases,
    CasesHistory,
    Comment,
    CommercialActHistory,
    CommercialActItemDamageDetails,
    CommercialActItems,
    CommercialActPassengerFlights,
    CommercialActPassengers,
    CommercialActs,
    Currency,
    CurrencyTranslation,
    DamageReasonTranslation,
    DamageReasons,
    DamagedBaggage,
    DamagedBaggageContents,
    DamagedBaggageDamageInfo,
    DamagedBaggageDamageInfoPhotos,
    DamagedBaggageFlights,
    DamagedBaggageHistory,
    DamagedBaggageNumbers,
    DamagedBaggageOwners,
    DamagedBaggagePassengerFlights,
    DamagedBaggagePlacePhotos,
    DamagedBaggagePlaces,
    DamagedBaggageProperties,
    DamagedBaggagePropertiesBasicElement,
    DamagedBaggagePropertiesExternalElement,
    DamagedBaggageRequesters,
    DamagedBaggageSettings,
    DamagedCommercialActCheckResults,
    DataProtectionKeys,
    DelayCodeTranslation,
    DelayCodes,
    DocumentAttachment,
    DocumentAttachmentVersion,
    DocumentsLinks,
    ExternalSystemSourceDocuments,
    MessageNotificationChannel,
    Messages,
    OrganizationAdditionalContacts,
    OrganizationNotification,
    OrganizationNotificationChannel,
    OrganizationUser,
    OrganizationUsersAirports,
    OrganizationUsersCarriers,
    Organizations,
    OrganizationsAirports,
    OrganizationsCarriers,
    PhotoPacketItems,
    PhotoPackets,
    PrintFormHeaderFooter,
    QuickDeliveries,
    QuickDeliveryBaggagePlaces,
    QuickDeliveryFlights,
    QuickDeliveryHistory,
    SupportRequestNumber,
    ThingCategories,
    ThingCategoryTranslation,
    ThingFlights,
    ThingPassengers,
    ThingPhotos,
    ThingProperties,
    ThingPropertiesBasicElement,
    ThingPropertiesExternalElement,
    Things,
    ThingsHistory,
    ThingsMatches,
    ThingsMatchingCoefficients,
    UnclaimedBaggageDeliveryFlights,
    UnclaimedCommercialActCheckResults,
    UserActionsHistory,
);
